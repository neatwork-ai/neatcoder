import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { ChatTreeViewProvider } from "../chat/providers";
import { chats } from "../extension";
import { getChat } from "../utils/utils";

type ChatInfo = {
  chatId: string;
  chatTitle: string;
};

const chatMap: Map<string, ChatInfo> = new Map();

export function setupChatWatcher(
  chatTreeViewProvider: ChatTreeViewProvider
): vscode.FileSystemWatcher {
  console.log("setting up watcher");
  const watcher = vscode.workspace.createFileSystemWatcher(
    "**/.neat/chats/*.json"
  );
  console.log("set up watcher!");

  watcher.onDidChange(async (uri: vscode.Uri) => {
    console.log("onDidChange triggered", uri.path);
    try {
      const chat = await getChat(uri);
      console.log(`The chat: ${chat}`);
      chats.insertChat(chat);
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
      console.log(`The chat: ${chat}`);
      chatMap.set(uri.toString(), {
        chatId: chat.sessionId,
        chatTitle: chat.title,
      });
      chats.insertChat(chat);
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
      chats.removeChat(chatInfo.chatId);
      chatMap.delete(uriString);
    } catch (error) {
      console.error("Failed to remove chat state:", error);
      throw new Error(`Failed to remove chat state from ${uri.path}: ${error}`);
    }
  });

  return watcher;
}
