import * as wasm from "../../pkg/neatcoder";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import {
  getOrCreateConfigPath,
  getOrInitConfig,
  getOrSetModelVersion,
  storeChat,
} from "../utils/utils";
import { setWebviewContent } from "./webview";
import { promptLLM } from "./handlers";
import { ChatProvider, activePanels } from ".";
import { v4 as uuidv4 } from "uuid";
import { makeRequest } from "../utils/httpClient";

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
  const sessionId = uuidv4();
  const chat = new wasm.Chat(sessionId, "TODO");
  chat.addModel(modelVersion!);
  storeChat(chat);

  // Setup event listeners and corresponding handlers
  panel.webview.onDidReceiveMessage(
    async (message) => {
      switch (message.command) {
        case "promptLLM":
          // Now, when we call buildOpenAIRequest, we pass along the
          // panel so it knows which panel sent the message
          chat.setMessages(message.msgs); // TODO: Move to addMessage to reduce communication overhead
          storeChat(chat);
          const msgs: Array<wasm.Message> = message.msgs;
          const isFirst = msgs.length === 1 ? true : false;

          console.log(`Is first? ${isFirst}`);

          promptLLM(panel, message);

          if (isFirst) {
            await chat.setTitle(makeRequest);
            storeChat(chat);

            // Change the title in the config
            let config = getOrInitConfig();

            const chatEntry = {
              id: chat.sessionId,
              title: chat.title,
            };

            console.log(`The new title: ${chatEntry.title}`);

            console.log(`Is config.chats? ${config?.chats}`);
            if (config?.chats) {
              const isChatEntryExist = config.chats.some(
                (chat) => chat.id === chatEntry.id
              );

              console.log(`Is isChatEntryExist? ${isChatEntryExist}`);
              if (!isChatEntryExist) {
                config = {
                  ...config,
                  chats: [...config.chats, chatEntry],
                };
              } else {
                console.log("Updating title.");
                const chatIndexToUpdate = config.chats.findIndex(
                  (chat) => chat.id === chatEntry.id
                );

                console.log(`chatIndexToUpdate? ${chatIndexToUpdate}`);
                if (chatIndexToUpdate !== -1) {
                  // Chat entry with the specified ID exists; update its properties
                  config = {
                    ...config,
                    chats: config.chats.map((chat, index) =>
                      index === chatIndexToUpdate
                        ? { ...chat, ...chatEntry }
                        : chat
                    ),
                  };
                } else {
                  throw new Error(`Failed to update title in the config file`);
                }
              }
            } else {
              console.log("Adding new title..");
              config = {
                ...config,
                chats: [chatEntry],
              };
            }

            console.log("Persisting changes");
            // Persist changes to the config - TODO: centralize this logic
            let configPath = getOrCreateConfigPath();
            const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
            fs.writeFileSync(configPath, updatedContent);
          }

          break;

        case "saveChat":
          // Store when GPT answer is complete
          chat.setMessages(message.msgs); // TODO: Move to addMessage to reduce communication overhead
          storeChat(chat);
          break;
        case "getUserId":
          const userId = vscode.workspace.getConfiguration().get<string>('NeatworkAi.neatcoder.userId');
          panel.webview.postMessage({ command: 'userId', userId: userId });
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
