import fetch from "node-fetch";
import * as fs from "fs";
import * as vscode from "vscode";
import * as path from "path";
import { getOrSetApiKey, getRoot } from "./utils";
import { TextDocument } from "vscode";
import { streamCode } from "./commands/streamCode";
import * as https from "https";
import * as url from "url";
import { logger } from "./logger";
import { startLoading, stopLoading } from "./statusBar";

let isProcessing = false;
let isCodeBlock = false;
let isCodeBlockMaybeEnding = false;
let waitingForNewline = false;

/**
 * This function makes an HTTP POST request to the OpenAI API
 * to get the completion suggestions based on the input body.
 * Note that this function will return the response in full.
 *
 * @param {string} body - The request body which possibly contains the text to be completed.
 * @return {Promise<object>} - A promise that resolves to the response object from the OpenAI API.
 */
export async function makeRequest(body: string): Promise<object> {
  const apiKey = getOrSetApiKey();

  try {
    const response = await fetch("https://api.openai.com/v1/chat/completions", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${apiKey}`,
      },
      body,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return (await response.json()) as object;
  } catch (error) {
    console.error("Error making request:", error);
    throw error;
  }
}

/**
 * This function initiates a streaming request to the OpenAI API.
 * It handles the streaming response, processes the received chunks of data,
 * manages the state variables related to code block streaming, and writes logs to files.
 *
 * @param {string} body - The request body which contains the text to be completed.
 * @param {TextDocument} activeTextDocument - The active text document in VS Code where the streamed data might be used.
 * @return {Promise<void>} - A promise that resolves when the streaming process completes successfully.
 */
export async function makeStreamingRequest(
  body: string,
  activeTextDocument: TextDocument
): Promise<void> {
  cleanup();

  return new Promise((resolve, reject) => {
    let responseLogRaw: string[] = [];
    let responseLog: string[] = [];
    let streamedTokens = 0;

    let messageBuffer = new MessageBuffer();

    const apiKey = getOrSetApiKey();
    try {
      const urlString = "https://api.openai.com/v1/chat/completions";
      const parsedUrl = url.parse(urlString);

      const options = {
        ...parsedUrl,
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${apiKey}`,
        },
      };

      const req = https.request(options, async (res) => {
        console.log(`STATUS: ${res.statusCode}`);
        res.setEncoding("utf8");
        res.pause();

        res.on("readable", async () => {
          if (isProcessing) {
            return;
          }

          isProcessing = true;
          let chunk;
          while (null !== (chunk = res.read())) {
            // Assuming 'chunk' is a Buffer, convert it to a string
            const chunkString = chunk.toString("utf8");

            // Processes chunk and returns valid messages
            const messages = messageBuffer.process(chunkString);

            for (const message of messages) {
              console.log(message);

              if (message === "[DONE]") {
                stopLoading();
                cleanup();
                isProcessing = false;

                if (streamedTokens === 0) {
                  vscode.window.showErrorMessage(
                    `Error: LLM failed to produce a coherent code block. Please try again.`
                  );
                  reject("Error: LLM failed to produce a coherent code block.");
                  return;
                }

                vscode.window.showInformationMessage("Streamed code finished.");
                console.log(`[INFO] Streaming process completed.`);

                resolve(); // Resolves the promise
                return;
              }

              const json = JSON.parse(message); // JSONify response

              // Here `json` will be an individual message object
              // You can then access json.choices[0].delta.content and proceed with your existing logic
              const token = json.choices[0].delta.content;

              if (token === null || token === undefined) {
                // TODO: If finish_reason === "stop" then return, else continue...
                // Skipping
                continue;
              }

              responseLog.push(token);

              writeLogs(responseLog);

              if (isCodeBlock) {
                if (checkIfCodeBlockMaybeEnding(token)) {
                  // If we get `` then it means the end of the code block may
                  // be near so we signal that
                  isCodeBlockMaybeEnding = true;
                  continue;
                }

                if (checkIfCodeBlockEnds(token)) {
                  // Here we have gotten the confirmation that the code block
                  // is completed
                  cleanup();
                  continue;
                }

                isCodeBlockMaybeEnding = false;

                if (checkForStreamStartSignal(token)) {
                  console.log(`[INFO] Stream is about to start.`);
                  waitingForNewline = false;
                  startLoading("Streaming");
                  continue;
                }

                if (checkIfCanStream()) {
                  console.log(`[INFO] Streaming token: ${token}`);
                  await streamCode(token, activeTextDocument);
                  streamedTokens += 1;
                }
              } else {
                if (checkIfCodeBlockIsStarting(token)) {
                  logger.appendLine(`[INFO] Starting Code Stream: ${token}`);
                  prepareStartStreaming();
                } else {
                  isCodeBlockMaybeEnding = false;
                }
              }
            }
          }
          isProcessing = false;
        });

        res.on("end", () => {
          cleanup();
          isProcessing = false;
          stopLoading();
          console.log("No more data in response.");
          resolve();
        });
      });

      req.on("error", (e) => {
        cleanup();
        isProcessing = false;
        console.error(`Problem with request: ${e.message}`);
        reject(e);
      });

      req.write(body);
      req.end();
    } catch (e) {
      console.error(e);
      cleanup();
      isProcessing = false;
      reject(e);
    }
  });
}

/**
 * This function prepares the environment to start streaming data by
 * setting the appropriate state variables.
 */
function prepareStartStreaming() {
  isCodeBlockMaybeEnding = false;
  isCodeBlock = true;
  waitingForNewline = true;
}

/**
 * This function resets the state variables used in the streaming process
 * to their initial values.
 */
function cleanup() {
  isCodeBlock = false;
  isCodeBlockMaybeEnding = false;
  waitingForNewline = false;
}

/**
 * This function checks whether a code block is ending based on the current token
 * and the state of the `isCodeBlockMaybeEnding` and `isCodeBlock` variables.
 *
 * @param {any} token - The current token or string being processed.
 * @return {boolean} - True if a code block is ending, false otherwise.
 */
function checkIfCodeBlockEnds(token: any): boolean {
  const check1 = isCodeBlockMaybeEnding === true && token[0] === "`";
  const check2 = token === "```" && isCodeBlock;

  return check1 || check2;
}

/**
 * This function checks if a code block might be ending soon, based on the
 * current token and the state of the `isCodeBlock` variable.
 *
 * @param {any} token - The current token or string being processed.
 * @return {boolean} - True if a code block might be ending soon, false otherwise.
 */
function checkIfCodeBlockMaybeEnding(token: any): boolean {
  return token === "``" && isCodeBlock;
}

/**
 * This function checks for a signal in the response data that indicates the
 * start of a stream, based on the current token and the state variables.
 *
 * @param {any} token - The current token or string being processed.
 * @return {boolean} - True if a stream start signal is detected, false otherwise.
 */
function checkForStreamStartSignal(token: any): boolean {
  return isCodeBlock && waitingForNewline && token === "\n";
}

/**
 * This function checks whether the script can continue streaming data
 * based on the current state variables.
 *
 * @return {boolean} - True if the script can continue streaming data, false otherwise.
 */
function checkIfCanStream(): boolean {
  return isCodeBlock && !waitingForNewline;
}

/**
 * This function checks if a code block is starting based on the current token
 * and the state of the `isCodeBlock` variable.
 *
 * @param {any} token - The current token or string being processed.
 * @return {boolean} - True if a code block is starting, false otherwise.
 */
function checkIfCodeBlockIsStarting(token: any): boolean {
  return !isCodeBlock && token === "```";
}

// TODO: Produce this only in debug mode
function writeRawLogs(responseLogRaw: string[]) {
  try {
    const root = getRoot();
    const folderPath = path.join(root, "responseLogRaw.json");
    fs.writeFile(folderPath, JSON.stringify(responseLogRaw, null, 2), (err) => {
      if (err) {
        console.error("Error writing to file", err);
      } else {
      }
    });
  } catch (error) {
    console.error(error);
    throw error;
  }
}

// TODO: Produce this only in debug mode
function writeLogs(responseLog: string[]) {
  try {
    const root = getRoot();
    const folderPath = path.join(root, "responseLog.json");
    fs.writeFile(folderPath, JSON.stringify(responseLog, null, 2), (err) => {
      if (err) {
        console.error("Error writing to file", err);
      } else {
      }
    });
  } catch (error) {
    console.error(error);
    throw error;
  }
}

class MessageBuffer {
  private buffer: string;

  constructor() {
    this.buffer = "";
  }

  private append(chunk: string): void {
    this.buffer += chunk;
  }

  private flush(index: number): string {
    const message = this.buffer.substring(0, index);
    this.buffer = this.buffer.substring(index + 2);
    return message;
  }

  process(chunkString: string): string[] {
    // 1. Attach incoming messages to the buffer
    this.append(chunkString);

    let index: number;
    let flushedMessages: string[] = [];

    while ((index = this.buffer.indexOf("\n\n")) >= 0) {
      const message = this.flush(index);

      if (message.startsWith("data: ")) {
        try {
          const payload = JSON.parse(message.substring(6)); // TODO: this is redundant?
          flushedMessages.push(JSON.stringify(payload));
        } catch (err) {
          let payload = message.substring(6);
          if (payload === "[DONE]") {
            flushedMessages.push(payload);
          } else {
            console.error(
              `Unable to parse message in HTTP stream into JSON object: ${message}`,
              err
            );
          }
        }
      } else {
        throw new Error(`Unable to parse message in HTTP stream: ${message}`);
      }
    }

    return flushedMessages;
  }
}
