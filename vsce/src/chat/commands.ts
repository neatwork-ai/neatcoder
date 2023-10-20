import * as wasm from "../../pkg/neatcoder";
import * as vscode from "vscode";
import * as path from "path";
import { storeChat } from "../utils/utils";
import { setWebviewContent } from "./webview";
import { promptLLM } from "./handlers";
import { activePanels, chats } from ".";

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

  const newChat = new wasm.Chat();
  storeChat("TODO", newChat);
  chats.insertChat(newChat);

  // Setup event listeners and corresponding handlers
  panel.webview.onDidReceiveMessage(
    (message) => {
      switch (message.command) {
        case "promptLLM":
          // Now, when we call buildOpenAIRequest, we pass along the
          // panel so it knows which panel sent the message
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
        chats.removeChat("TODO");
      }
    }
  });
}
