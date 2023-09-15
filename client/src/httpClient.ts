import fetch from "node-fetch";
import * as fs from "fs";
import * as path from "path";
import { getOrSetApiKey, getRoot } from "./utils";
import { Position, TextDocument, window, OutputChannel } from "vscode";
import { streamCode } from "./commands/streamCode";
import * as https from "https";
import * as url from "url";

let isProcessing = false;
let isCodeBlock = false;
let isCodeBlockMaybeEnding = false;
let waitingForNewline = false;

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

export async function makeStreamingRequest(
  body: string,
  activeTextDocument: TextDocument,
  logger: OutputChannel
): Promise<void> {
  return new Promise((resolve, reject) => {
    let responseLogRaw: string[] = [];
    let responseLog: string[] = [];

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
            logger.appendLine(`Stopping the spam`);
            console.log(`Stopping the spam`);
            return;
          }

          isProcessing = true;
          let chunk;
          while (null !== (chunk = res.read())) {
            console.log(`Chunk: ${chunk}`);
            // const chunkAsString = chunk.toString("utf8"); // .substring(6); // removes the prefix "data: "
            // const messages: string[] = chunkAsString.split("\n\n");

            // Assuming 'chunk' is a Buffer, convert it to a string
            const chunkAsString0 = chunk.toString("utf8");

            responseLogRaw.push(chunkAsString0);

            writeRawLogs(responseLogRaw);

            // Split the chunk by double newline to get individual messages
            const chunkAsString = chunk.toString("utf8"); // .substring(6); // removes the prefix "data: "
            const messages: string[] = chunkAsString.split("\n\n");

            for (const message of messages) {
              if (message.startsWith("data: ") && message.trim().length > 0) {
                const rawData = message.substring(6); // remove 'data: ' prefix

                if (rawData.trim() === "[DONE]") {
                  console.log(`[INFO] Streaming process completed.`);
                  cleanup();
                  isProcessing = false;

                  resolve(); // Resolves the promise
                  return;
                }

                const json = JSON.parse(rawData); // JSONify response

                // Here `json` will be an individual message object
                // You can then access json.choices[0].delta.content and proceed with your existing logic
                const token = json.choices[0].delta.content;

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
                    logger.appendLine(`[INFO] Streaming END`);
                    console.log(`[INFO] Streaming END`);
                    // Ending the stream listening
                    cleanup();
                    continue;
                  }

                  // TODO: Consider where to put this...
                  isCodeBlockMaybeEnding = false;

                  if (checkForStreamStartSignal(token)) {
                    console.log(
                      `[INFO] Stream is about to start. Received token: ${token}`
                    );

                    waitingForNewline = false;
                    continue;
                  }

                  if (checkIfCanStream()) {
                    console.log(`[INFO] Streaming token: ${token}`);
                    await streamCode(token, activeTextDocument, logger);
                  }
                } else {
                  if (checkIfCodeBlockIsStarting(token)) {
                    logger.appendLine(`[INFO] START Streaming: ${token}`);
                    console.log(`[INFO] START Streaming: ${token}`);
                    prepareStartStreaming();
                  } else {
                    console.log(`Skipping: ${token}`);
                    isCodeBlockMaybeEnding = false;
                  }
                }
              }
            }
          }
          isProcessing = false;
        });

        res.on("end", () => {
          cleanup();
          isProcessing = false;
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

function prepareStartStreaming() {
  isCodeBlockMaybeEnding = false;
  isCodeBlock = true;
  waitingForNewline = true;
}

function cleanup() {
  isCodeBlock = false;
  isCodeBlockMaybeEnding = false;
  waitingForNewline = false;
}

function checkIfCodeBlockEnds(token: any): boolean {
  const check1 = isCodeBlockMaybeEnding === true && token[0] === "`";
  const check2 = token === "```" && isCodeBlock;

  return check1 || check2;
}

function checkIfCodeBlockMaybeEnding(token: any): boolean {
  return token === "``" && isCodeBlock;
}

function checkForStreamStartSignal(token: any): boolean {
  return isCodeBlock && waitingForNewline && token === "\n";
}

function checkIfCanStream(): boolean {
  return isCodeBlock && !waitingForNewline;
}

function checkIfCodeBlockIsStarting(token: any): boolean {
  return !isCodeBlock && token === "```";
}

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

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
