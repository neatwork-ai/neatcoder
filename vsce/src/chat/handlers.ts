import * as vscode from "vscode";
import { window } from "vscode";
import { getOrSetApiKey } from "../utils";
import * as wasm from "../../pkg/neatcoder";
import * as https from "https";
import * as http from "http";
import * as url from "url";
import { MessageBuffer } from "../utils/httpClient";
import { getLLMParams } from "../utils/utils";

export async function buildRequest(
  msgs: Array<wasm.Message>,
  stream: boolean
): Promise<[any, any]> {
  const apiKey = await getOrSetApiKey();

  try {
    console.log("Messages: " + JSON.stringify(msgs.map((msg) => msg.payload)));
    let llmParams = await getLLMParams();

    const body = wasm.requestBody(
      msgs.map((msg) => msg.payload),
      llmParams,
      stream
    );
    return [apiKey, body];
  } catch (err) {
    console.error(`Failed to build request: ${err}`);
    throw new Error((err as Error).message);
  }
}

export async function promptLLM(
  webviewPanel: vscode.WebviewPanel,
  message: any
): Promise<void> {
  const msgs: Array<wasm.Message> = message.msgs;
  const stream = message.stream;

  const [apiKey, body] = await buildRequest(msgs, stream);

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
        if (!(res.statusCode === 202 || res.statusCode === 200)) {
          const statusMessage =
            http.STATUS_CODES[res.statusCode!] || "Unknown status code";

          console.log(`STATUS: ${res.statusCode} ${statusMessage}`);
          // Here the use of `window` and `webviewPanel` assumes this is within a VS Code extension
          window.showErrorMessage(
            `HTTP error: STATUS: ${res.statusCode} ${statusMessage}`
          );

          reject(
            new Error(`HTTP error: STATUS: ${res.statusCode} ${statusMessage}`)
          );
          return; // Stop further processing
        }

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
    } catch (err) {
      console.error(`Failed to prompt the LLM: ${err}`);
      reject(err);
    }
  });
}
