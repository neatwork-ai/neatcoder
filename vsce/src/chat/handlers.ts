import * as vscode from "vscode";
import { getOrSetApiKey } from "../utils";
import * as wasm from "../../pkg/neatcoder";

export async function buildOpenAIRequest(
  webviewPanel: vscode.WebviewPanel,
  message: any
) {
  const msgs = message.msgs;
  const stream = message.stream;

  const [apiKey, body] = buildRequest(msgs, stream);

  // Respond back to the webview
  webviewPanel.webview.postMessage({
    command: "buildOpenAIRequest",
    apiKey: apiKey,
    body: body,
  });
}

export function buildRequest(
  msgs: Array<wasm.OpenAIMsg>,
  stream: boolean
): [any, any] {
  const apiKey = getOrSetApiKey();

  try {
    const body = wasm.requestBody(msgs, stream);
    return [apiKey, body];
  } catch (error) {
    console.error("An error occurred:", error);
    throw new Error((error as Error).message);
  }
}
