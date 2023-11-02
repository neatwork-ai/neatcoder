import * as vscode from "vscode";
import * as fs from "fs";
import { ChatEntry, ChatProvider } from "../chat/providers";
import {
  getChat,
  getConfigIfAny,
  getOrCreateConfigPath,
  getOrInitConfig,
} from "../utils/utils";

const chatMap: Map<string, ChatEntry> = new Map();

export function setupChatWatcher(
  chatTreeViewProvider: ChatProvider
): vscode.FileSystemWatcher {
  const watcher = vscode.workspace.createFileSystemWatcher(
    "**/.neat/chats/*.json"
  );

  watcher.onDidChange(async (uri: vscode.Uri) => {
    // No-Op
    // console.log("onDidChange triggered", uri.path);
    // try {
    //   const chat = await getChat(uri);
    //   // chats.insertChat(chat); // TODO: Lazily read chats on webview open
    //   chatTreeViewProvider.refresh();
    // } catch (error) {
    //   vscode.window.showErrorMessage(
    //     `[ERROR] Failed to read chat file ${uri}. Error: ${error}`
    //   );
    //   console.error("Failed to update chat state:", error);
    //   throw new Error(`Failed to update chat state ${uri.path}: ${error}`);
    // }
  });

  watcher.onDidCreate(async (uri: vscode.Uri) => {
    console.log("onDidCreate triggered", uri.path);
    try {
      const chat = await getChat(uri);

      const chatEntry = {
        id: chat.sessionId,
        title: chat.title,
      };

      chatMap.set(uri.toString(), chatEntry);

      // Add the chat entry to the config
      let config = getOrInitConfig();

      if (config?.chats) {
        const isChatEntryExist = config.chats.some(
          (chat) => chat.id === chatEntry.id
        );

        if (!isChatEntryExist) {
          config = {
            ...config,
            chats: [...config.chats, chatEntry],
          };
        }
      } else {
        config = {
          ...config,
          chats: [chatEntry],
        };
      }

      // Persist changes to the config - TODO: centralize this logic
      let configPath = getOrCreateConfigPath();
      const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
      fs.writeFileSync(configPath, updatedContent);

      chatTreeViewProvider.refresh(); // Redundant?
    } catch (error) {
      vscode.window.showErrorMessage(
        `[ERROR] Failed to read new chat file ${uri}. Error: ${error}`
      );

      throw new Error(`Failed to create chat state from ${uri.path}: ${error}`);
    }
  });

  watcher.onDidDelete((uri: vscode.Uri) => {
    try {
      const uriString = uri.toString();
      const chatEntry = chatMap.get(uriString)!;
      chatMap.delete(uriString);

      let config = getOrInitConfig();

      const chatIndexToDelete = config.chats.findIndex(
        (chat) => chat.id === chatEntry.id
      );

      if (chatIndexToDelete !== -1) {
        config = {
          ...config,
          chats: [
            ...config.chats.slice(0, chatIndexToDelete),
            ...config.chats.slice(chatIndexToDelete + 1),
          ],
        };

        // Persist changes to the config - TODO: centralize this logic

        let configPath = getOrCreateConfigPath();
        const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
        fs.writeFileSync(configPath, updatedContent);
      }

      chatTreeViewProvider.refresh();
    } catch (error) {
      console.error("Failed to remove chat state:", error);
      throw new Error(`Failed to remove chat state from ${uri.path}: ${error}`);
    }
  });

  return watcher;
}
