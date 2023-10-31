import * as wasm from "../../pkg/neatcoder";
import * as vscode from "vscode";
import * as path from "path";
import { getOrSetModelVersion, storeChat } from "../utils/utils";
import { setWebviewContent } from "./webview";
import { promptLLM } from "./handlers";
import { activePanels } from ".";

export let panelCounter = 1;

export async function initChat(
  context: vscode.ExtensionContext
): Promise<void> {
  const panel = vscode.window.createWebviewPanel(
    "chatPanel",
    `Chat ${panelCounter}`,
    vscode.ViewColumn.One,
    {
      enableScripts: true,
      retainContextWhenHidden: true,
      localResourceRoots: [
        vscode.Uri.file(
          path.join(context.extensionPath, "..", "webview", "build")
        ),
      ],
    }
  );

  panel.iconPath = vscode.Uri.file(
    path.join(context.extensionPath, "assets", "robot-32-30.png")
  );

  let modelVersion = await getOrSetModelVersion();

  const chat = new wasm.Chat("TODO");
  chat.addModel(modelVersion!);
  storeChat(chat);

  // Setup event listeners and corresponding handlers
  panel.webview.onDidReceiveMessage(
    (message) => {
      switch (message.command) {
        case "promptLLM":
          // Now, when we call buildOpenAIRequest, we pass along the
          // panel so it knows which panel sent the message
          chat.setMessages(message.msgs); // TODO: Move to addMessage to reduce communication overhead
          storeChat(chat);
          promptLLM(panel, message);
          break;
      }
    },
    undefined,
    context.subscriptions
  );

  setWebviewContent(panel, context);
  activePanels.set(panelCounter, panel);
  panelCounter++;

  panel.onDidDispose(() => {
    // Remove from active panels map when it's closed
    for (const [key, activePanel] of activePanels.entries()) {
      if (activePanel === panel) {
        activePanels.delete(key);
      }
    }
  });
}
