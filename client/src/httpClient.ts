import fetch from "node-fetch";
import * as fs from "fs";
import * as path from "path";
import { getOrSetApiKey, getRoot } from "./utils";
import { Position, TextDocument, window, OutputChannel } from "vscode";
import { streamCode } from "./commands/streamCode";

let isProcessing = false;
let isStreaming = true;
let isEndClose = false;
let waitingForNewline = false;
let startDelimiter = false;

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

// TODO
// export async function makeStreamingRequest(
//   body: string,
//   activeTextDocument: TextDocument,
//   logger: OutputChannel
// ): Promise<void> {
//   const apiKey = getOrSetApiKey();

//   return new Promise((resolve, reject) => {
//     const options = {
//       method: "POST",
//       headers: {
//         "Content-Type": "application/json",
//         Authorization: `Bearer ${apiKey}`,
//       },
//     };

//     const req = https.request(
//       "https://api.openai.com/v1/chat/completions",
//       options,
//       (res) => {
//         res.on("data", (d) => {
//           console.log(d);
//           logger.appendLine(`[INFO] Streamed: ${d}`);
//           // call --> await streamCode(..);
//         });

//         res.on("end", () => {
//           resolve(); // Resolves the promise when the stream ends
//         });
//       }
//     );

//     req.on("error", (e) => {
//       reject(e); // Rejects the promise if there's an error
//     });

//     req.write(body); // Write the request body
//     req.end();
//   });
// }

export function makeStreamingRequest(
  body: string,
  activeTextDocument: TextDocument,
  logger: OutputChannel
): Promise<void> {
  return new Promise(async (resolve, reject) => {
    let responseLog: string[] = [];

    const apiKey = getOrSetApiKey();
    try {
      const response = await fetch(
        "https://api.openai.com/v1/chat/completions",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${apiKey}`,
          },
          body,
        }
      );

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      if (response.body) {
        response.body
          .on("data", async (chunk: any) => {
            while (isProcessing) {
              await new Promise((resolve) => setTimeout(resolve, 0));
            }

            lock();

            // Assuming 'chunk' is a Buffer, convert it to a string
            const chunkAsString0 = chunk.toString("utf8");
            responseLog.push(chunkAsString0);

            try {
              const root = getRoot();
              const folderPath = path.join(root, "responseLog.json");
              fs.writeFile(
                folderPath,
                JSON.stringify(responseLog, null, 2),
                (err) => {
                  if (err) {
                    console.error("Error writing to file", err);
                  } else {
                  }
                }
              );
            } catch (error) {
              console.error(error);
              throw error;
            }

            // Split the chunk by double newline to get individual messages
            const chunkAsString = chunk.toString("utf8"); // .substring(6); // removes the prefix "data: "
            const messages: string[] = chunkAsString.split("\n\n");

            for (const message of messages) {
              if (message.startsWith("data: ") && message.trim().length > 0) {
                // Remove 'data: ' prefix and parse JSON
                const json = JSON.parse(message.substring(6)); // remove 'data: ' prefix

                // Here `json` will be an individual message object
                // You can then access json.choices[0].delta.content and proceed with your existing logic
                const token = json.choices[0].delta.content;

                if (!startDelimiter && token === "```") {
                  logger.appendLine(`[INFO] Streamed START: ${token}`);
                  prepareStartStreaming();
                  unlock();
                  return;
                }
                if (!startDelimiter) {
                  logger.appendLine(`[INFO] Skipping: ${token}`);
                  console.log(`Skipping: ${token}`);
                  unlock();
                  // If it hasn't started the code block then ignore
                  return;
                } else {
                  if (token === "``") {
                    // If we get `` then it means the end of the code block may
                    // be near so we signal that
                    isEndClose = true;
                    unlock();
                    return;
                  }

                  if (isEndClose === true && token[0] === "`") {
                    // Here we have gotten the confirmation that the code block
                    // is completed
                    logger.appendLine(`[INFO] Streamed END`);
                    // Ending the stream listening
                    isStreaming = false;
                    isProcessing = false;
                    resolve();
                    return;
                  }

                  if (token === "```") {
                    /// This is an alternative end of the code block, in case
                    // the three delimiter chars come in the same streamed json.
                    logger.appendLine(`[INFO] Streamed END`);
                    // Ending the stream listening
                    isStreaming = false;
                    isProcessing = false;
                    resolve();
                    return;
                  }

                  isEndClose = false;

                  console.log(`isStreaming is: ${isStreaming}`);
                  console.log(`waitingForNewline is: ${waitingForNewline}`);

                  if (isStreaming) {
                    if (waitingForNewline && token === "\n") {
                      waitingForNewline = false;
                      isProcessing = false;
                      return;
                    }
                    if (!waitingForNewline) {
                      await streamCode(token, activeTextDocument, logger);
                      isProcessing = false;
                    }
                  }
                }
              }
            }
          })
          .on("error", (err: any) => {
            reject(`Error reading the stream: ${err}`);
          })
          .on("end", () => {
            resolve();
          });
      } else {
        reject("Response body is null");
      }
    } catch (error) {
      reject(`Error during request: ${error}`);
    }
  });
}

function prepareStartStreaming() {
  startDelimiter = true;
  waitingForNewline = true;
  isProcessing = false;
}

function unlock() {
  isProcessing = false;
}

function lock() {
  isProcessing = false;
}
