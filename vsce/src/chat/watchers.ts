import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { ChatTreeViewProvider } from "../chat/providers";
import { getChat } from "../utils/utils";

type ChatInfo = {
  chatId: string;
  chatTitle: string;
};

const chatMap: Map<string, ChatInfo> = new Map();

export function setupChatWatcher(
  chatTreeViewProvider: ChatTreeViewProvider
): vscode.FileSystemWatcher {
  const watcher = vscode.workspace.createFileSystemWatcher(
    "**/.neat/chats/*.json"
  );

  watcher.onDidChange(async (uri: vscode.Uri) => {
    console.log("onDidChange triggered", uri.path);
    try {
      const chat = await getChat(uri);
      // chats.insertChat(chat); // TODO: Lazily read chats on webview open
      chatTreeViewProvider.refresh();
    } catch (error) {
      vscode.window.showErrorMessage(
        `[ERROR] Failed to read chat file ${uri}. Error: ${error}`
      );

      console.error("Failed to update chat state:", error);
      throw new Error(`Failed to update chat state ${uri.path}: ${error}`);
    }
  });

  watcher.onDidCreate(async (uri: vscode.Uri) => {
    console.log("onDidCreate triggered", uri.path);
    try {
      const chat = await getChat(uri);
      chatMap.set(uri.toString(), {
        chatId: chat.sessionId,
        chatTitle: chat.title,
      });
      // chats.insertChat(chat); // TODO: Lazily read chats on webview open
      chatTreeViewProvider.refresh();
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
      const chatInfo = chatMap.get(uriString)!;
      // chats.removeChat(chatInfo.chatId); TODO: Lazily remove chats..
      chatMap.delete(uriString);
    } catch (error) {
      console.error("Failed to remove chat state:", error);
      throw new Error(`Failed to remove chat state from ${uri.path}: ${error}`);
    }
  });

  return watcher;
}
