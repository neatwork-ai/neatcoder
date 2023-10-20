import * as vscode from "vscode";
import { getOrSetApiKey } from "../utils";
import * as wasm from "../../pkg/neatcoder";
import * as https from "https";
import * as url from "url";
import { MessageBuffer } from "../utils/httpClient";

// export async function buildOpenAIRequest(
//   webviewPanel: vscode.WebviewPanel,
//   message: any
// ) {
//   console.log("Building OpenAI Request body");

//   const msgs = message.msgs;
//   const stream = message.stream;

//   const [apiKey, body] = buildRequest(msgs, stream);

//   console.log("Built body: " + JSON.stringify(body));

//   // Respond back to the webview
//   webviewPanel.webview.postMessage({
//     command: "buildOpenAIRequest",
//     apiKey: apiKey,
//     body: body,
//   });
// }

// export async function promptLLM2(
//   webviewPanel: vscode.WebviewPanel,
//   message: any
// ) {
//   const msgs: Array<wasm.Message> = message.msgs;
//   const stream = message.stream;

//   const [apiKey, body] = buildRequest(msgs, stream);

//   console.log("MSG: " + JSON.stringify(msgs));
//   console.log("Body: " + JSON.stringify(body));

//   const response = await fetch("https://api.openai.com/v1/chat/completions", {
//     method: "POST",
//     headers: {
//       Authorization: `Bearer ${apiKey}`,
//       "Content-Type": "application/json",
//     },
//     body: JSON.stringify(body),
//   });

//   const reader = response.body?.getReader();

//   if (reader) {
//     while (true) {
//       const { done, value } = await reader.read();

//       if (done) {
//         break;
//       }

//       if (value) {
//         // Send this chunk to the webview
//         webviewPanel.webview.postMessage({
//           command: "dataChunk",
//           chunk: new TextDecoder().decode(value),
//         });
//       }
//     }
//     // Indicate end of stream
//     webviewPanel.webview.postMessage({ command: "endOfStream" });
//   }
// }

export function buildRequest(
  msgs: Array<wasm.Message>,
  stream: boolean
): [any, any] {
  const apiKey = getOrSetApiKey();

  try {
    const body = wasm.requestBody(
      msgs.map((msg) => msg.payload),
      stream
    );
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
