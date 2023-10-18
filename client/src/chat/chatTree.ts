import * as vscode from "vscode";
import * as path from "path";
import { activePanels } from "../extension";

class ChatTreeItem extends vscode.TreeItem {
  constructor(public readonly chatId: string, public readonly label: string) {
    super(label, vscode.TreeItemCollapsibleState.None);
  }
}

export class ChatTreeViewProvider
  implements vscode.TreeDataProvider<ChatTreeItem>
{
  private _onDidChangeTreeData: vscode.EventEmitter<ChatTreeItem | undefined> =
    new vscode.EventEmitter<ChatTreeItem | undefined>();
  readonly onDidChangeTreeData: vscode.Event<ChatTreeItem | undefined> =
    this._onDidChangeTreeData.event;

  getTreeItem(element: ChatTreeItem): vscode.TreeItem {
    return element;
  }

  getChildren(element?: ChatTreeItem): Thenable<ChatTreeItem[]> {
    return Promise.resolve(this.getChatTreeItems());
  }

  // private getChatTreeItems(): ChatTreeItem[] {
  //   const chats = [];
  //   for (const [key] of activePanels.entries()) {
  //     chats.push(new ChatTreeItem(key, `Chat ${key}`));
  //   }
  //   console.log("ChatTreeItems: ", chats);
  //   return chats;
  // }

  private async getChatTreeItems(): Promise<ChatTreeItem[]> {
    const chats: ChatTreeItem[] = [];
    const chatsDir = vscode.Uri.file(
      vscode.workspace.rootPath + "/.neat/chats"
    );

    let directoryExists = true;
    try {
      await vscode.workspace.fs.stat(chatsDir);
    } catch {
      directoryExists = false;
    }

    if (directoryExists) {
      const chatFiles = await vscode.workspace.fs.readDirectory(chatsDir);

      chatFiles.forEach(([file, type]) => {
        if (type === vscode.FileType.File) {
          const chatName = path.basename(file, ".json"); // Extract name without extension
          chats.push(new ChatTreeItem(chatName, `${chatName}`));
        }
      });

      console.log("ChatTreeItems: ", chats);
    }

    return chats;
  }

  refresh(): void {
    this._onDidChangeTreeData.fire(undefined);
  }
}

const chatTreeViewProvider = new ChatTreeViewProvider();
vscode.window.registerTreeDataProvider("chatTreeView", chatTreeViewProvider);

// Update the tree view whenever a panel is created or closed
vscode.window.onDidChangeVisibleTextEditors(() => {
  chatTreeViewProvider.refresh();
});
