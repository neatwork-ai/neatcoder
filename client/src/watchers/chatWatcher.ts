import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { ChatTreeViewProvider } from "../chat/chatTree";

export function setupChatWatcher(
  chatTreeViewProvider: ChatTreeViewProvider
): vscode.FileSystemWatcher {
  const watcher = vscode.workspace.createFileSystemWatcher(
    "**/.neat/chats/*.json"
  );

  const refreshTree = () => chatTreeViewProvider.refresh();

  watcher.onDidChange(refreshTree);
  watcher.onDidCreate(refreshTree);
  watcher.onDidDelete(refreshTree);

  return watcher;
}
