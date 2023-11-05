import * as wasm from "../../pkg/neatcoder";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import {
  getChatById,
  getOrCreateConfigPath,
  getOrInitConfig,
  getOrSetModelVersion,
  getRoot,
  storeChat,
} from "../utils/utils";
import { setWebviewContent } from "./webview";
import { promptLLM } from "./handlers";
import { activePanels } from ".";
import { v4 as uuidv4 } from "uuid";
import { makeRequest } from "../utils/httpClient";
import { ChatItem } from "./providers";
import { ChatOperationQueue } from "./io";

export let panelCounter = 1;
// Instantiate the operation queue
const chatOperationQueue = new ChatOperationQueue();

export async function initChat(
  context: vscode.ExtensionContext
): Promise<void> {
  const panel = vscode.window.createWebviewPanel(
    "chatPanel",
    "Let's Chat",
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
  const chat = new wasm.Chat(sessionId, "Chat with Neat");
  chat.addModel(modelVersion!);

  // This will add the storeChat operation to the queue to be processed in order.
  chatOperationQueue.add(() => storeChat(chat));

  // Setup event listeners and corresponding handlers
  panel.webview.onDidReceiveMessage(
    async (message) => {
      await setupWebviewSockets(message, panel, chat);
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

export async function openChat(
  context: vscode.ExtensionContext,
  chatitem: ChatItem
): Promise<void> {
  const chatHistory = await getChatById(chatitem.id);

  const panel = vscode.window.createWebviewPanel(
    "chatPanel",
    chatHistory.title,
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

  // Setup event listeners and corresponding handlers
  panel.webview.onDidReceiveMessage(
    async (message) => {
      await setupWebviewSockets(message, panel, chatHistory);
    },
    undefined,
    context.subscriptions
  );

  setWebviewContent(panel, context, chatHistory);
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

const setupWebviewSockets = async (
  message: any,
  panel: vscode.WebviewPanel,
  chat: wasm.Chat
) => {
  switch (message.command) {
    case "promptLLM":
      // Now, when we call buildOpenAIRequest, we pass along the
      // panel so it knows which panel sent the message
      chat.setMessages(message.msgs); // TODO: Move to addMessage to reduce communication overhead
      chatOperationQueue.add(() => storeChat(chat));
      const msgs: Array<wasm.Message> = message.msgs;
      const isFirst = msgs.length === 1 ? true : false;

      promptLLM(panel, message);

      if (isFirst) {
        await chat.setTitle(makeRequest);
        chatOperationQueue.add(() => storeChat(chat));

        // Change the title in the config
        let config = getOrInitConfig();

        const chatEntry = {
          id: chat.sessionId,
          title: chat.title,
        };

        if (config?.chats) {
          const isChatEntryExist = config.chats.some(
            (chat) => chat.id === chatEntry.id
          );

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

            if (chatIndexToUpdate !== -1) {
              // Chat entry with the specified ID exists; update its properties
              config = {
                ...config,
                chats: config.chats.map((chat, index) =>
                  index === chatIndexToUpdate ? { ...chat, ...chatEntry } : chat
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

        panel.title = chatEntry.title;
      }

      break;

    case "saveChat":
      console.log("Received saveChat request.");

      console.log(`Messages: ${JSON.stringify(message.msgs)}`);
      // Store when GPT answer is complete
      chat.setMessages(message.msgs); // TODO: Move to addMessage to reduce communication overhead
      chatOperationQueue.add(() => storeChat(chat));
      break;
  }
};

export function removeChat(chatItem: ChatItem) {
  if (!chatItem) {
    vscode.window.showErrorMessage("Please select a schema to remove.");
    return;
  }

  const chatID = chatItem.id;
  const config = getOrInitConfig();

  // Filter out the chat entry with the matching id
  const updatedChats = config.chats.filter((chat) => chat.id !== chatID);

  // Update the config object
  config.chats = updatedChats;

  // Persist changes to the config - TODO: centralize this logic
  let configPath = getOrCreateConfigPath();
  const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
  fs.writeFileSync(configPath, updatedContent);

  // Construct the path to the chat file
  const root = getRoot();
  const schemaFilePath = path.join(root, ".neat", "chats", `${chatID}.json`);

  if (!fs.existsSync(schemaFilePath)) {
    vscode.window.showErrorMessage(`Chat file not found at ${schemaFilePath}`);
    return;
  }

  // Delete the chat file
  fs.unlinkSync(schemaFilePath);

  vscode.window.showInformationMessage(
    `Chat named ${chatItem.label} removed successfully.`
  );
}
