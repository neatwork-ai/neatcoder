// /// <reference types="./vscode.d.ts" />

import { Message } from "../wasm/neatcoderInterface";

export function promptLLM(msgs: Array<Message>, stream: boolean): ReadableStream {
  const { readable, writable } = new TransformStream();

  const writer = writable.getWriter();

  const vscode = acquireVsCodeApi();

  vscode.postMessage({
    command: 'promptLLM',
    msgs: msgs,
    stream: stream
  });

  window.addEventListener('message', (event) => {
    const message = event.data;

    if (message.command === 'dataChunk') {
      writer.write(message.chunk);
    } else if (message.command === 'endOfStream') {
      writer.close();
    }
  });

  return readable;
}


// export function buildOpenAIRequest(msgs: Array<Message>, stream: boolean): Promise<{ apiKey: any, body: any}> {
//   return new Promise((resolve) => {
//     const vscode = acquireVsCodeApi();

//     // Post the message to the VS Code extension
//     vscode.postMessage({
//       command: 'buildOpenAIRequest',
//       msgs: msgs,
//       stream: stream
//     });

//     console.log("Message successfully posted...")

//     function handleVscodeMessage(event: any) {
//       const message = event.data;

//       if (message.command === 'buildOpenAIRequest') {
//         window.removeEventListener('message', handleVscodeMessage);
//         console.log("Received message..")
//         resolve({ apiKey: message.apiKey, body: message.body });
//       }
//     }

//     window.addEventListener('message', handleVscodeMessage);
//   });
// }

export async function streamOpenAIResponse(apiKey: any, body: any) {
  const headers = {
      'Authorization': `Bearer ${apiKey}`,
      'Content-Type': 'application/json',
      // 'Accept': 'text/event-stream',  // This header is important for SSE.
  };

  try {
      const response = await fetch("https://api.openai.com/v1/chat/completions", {
          method: 'POST',
          headers: headers,
          body: JSON.stringify(body)
      });

      console.log("The body: " + JSON.stringify(body));

      if (!response.ok) {
          throw new Error(`OpenAI API call failed with status ${response.status}`);
      }

      if (!response.body) {
        throw new Error('Response body is null');
    }

      const reader = response.body.getReader();

      // This will be our text accumulator
      let text = "";

      return new ReadableStream({
          async start(controller) {
              while (true) {
                  const { done, value } = await reader.read();

                  if (done) break;

                  // Convert the Uint8Array data to a string
                  text += new TextDecoder().decode(value);

                  // Whenever we encounter a newline, we got a new token (or other data you're streaming)
                  const lines = text.split('\n');
                  for (let i = 0; i < lines.length - 1; i++) {
                      controller.enqueue(lines[i]);
                  }
                  // The last line is either empty or incomplete, so save it for later
                  text = lines[lines.length - 1];
              }
              controller.close();
          }
      });
  } catch (error) {
      console.error('Error calling OpenAI API:', error);
      throw error;
  }
}
