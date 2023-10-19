import * as wasm from "../../pkg/neatcoder";
import * as vscode from "vscode";
import * as path from "path";
import { activePanels, chats } from "../extension";
import { storeChat } from "../utils/utils";
import { setWebviewContent } from "./webview";

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

  setWebviewContent(panel, context);
  activePanels.set(panelCounter, panel);
  panelCounter++;

  // Listen for messages from this webview - TODO
  // panel.webview.onDidReceiveMessage(
  //   async (message) => {
  //     switch (message.command) {
  //       case "buildOpenAIRequest":
  //         const responseText = await handleOpenAIRequest(message.text);
  //         panel.webview.postMessage({
  //           command: "buildOpenAIRequest",
  //           text: responseText,
  //         });
  //         break;

  //       // ... handle other commands as needed
  //     }
  //   },
  //   undefined,
  //   context.subscriptions
  // );

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
