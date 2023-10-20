import * as vscode from "vscode";
import { getOrSetApiKey } from "../utils";
import * as wasm from "../../pkg/neatcoder";
import * as https from "https";
import * as url from "url";
import { MessageBuffer } from "../utils/httpClient";

export function buildRequest(
  msgs: Array<wasm.Message>,
  stream: boolean
): [any, any] {
  const apiKey = getOrSetApiKey();

  try {
    console.log("Messages: " + msgs.map((msg) => msg.payload));
    const body = wasm.requestBody(
      msgs.map((msg) => msg.payload),
      stream
    );
    console.log("B");
    return [apiKey, body];
  } catch (error) {
    console.error("An error occurred:", error);
    throw new Error((error as Error).message);
  }
}

export async function promptLLM(
  webviewPanel: vscode.WebviewPanel,
  message: any
): Promise<void> {
  const msgs: Array<wasm.Message> = message.msgs;
  const stream = message.stream;

  const [apiKey, body] = buildRequest(msgs, stream);

  return new Promise((resolve, reject) => {
    let messageBuffer = new MessageBuffer();

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
          let chunk;
          while (null !== (chunk = res.read())) {
            // Assuming 'chunk' is a Buffer, convert it to a string
            const chunkString = chunk.toString("utf8");

            // Processes chunk and returns valid messages
            const messages = messageBuffer.process(chunkString);

            for (const message of messages) {
              if (message === "[DONE]") {
                // Indicate end of stream
                webviewPanel.webview.postMessage({ command: "endOfStream" });
                continue;
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

              webviewPanel.webview.postMessage({
                command: "dataChunk",
                chunk: token,
              });
            }
          }
        });

        res.on("end", () => {
          console.log("No more data in response.");
          resolve();
        });
      });

      req.on("error", (e) => {
        console.error(`Problem with request: ${e.message}`);
        reject(e);
      });

      req.write(body);
      req.end();
    } catch (e) {
      console.error(e);
      reject(e);
    }
  });
}
