import * as vscode from "vscode";
import { getConfigIfAny } from "../utils/utils";

export type ChatEntry = {
  id: string;
  title: string;
};

export class ChatItem extends vscode.TreeItem {
  constructor(public readonly id: string, public readonly label: string) {
    super(label, vscode.TreeItemCollapsibleState.None);
  }
}

/**
 * Class implementing the vscode.TreeDataProvider interface to provide data for a tree view
 * in a VS Code extension. It manages the representation of chats.
 */
export class ChatProvider implements vscode.TreeDataProvider<ChatItem> {
  /**
   * Constructor initializes a new event emitter instance for tree data change notifications.
   */
  constructor() {
    this._onDidChangeTreeData = new vscode.EventEmitter();
    this.onDidChangeTreeData = this._onDidChangeTreeData.event;
  }

  /**
   * Private property holding an event emitter instance to notify about tree data changes.
   */
  private _onDidChangeTreeData: vscode.EventEmitter<
    ChatItem | undefined | null | void
  > = new vscode.EventEmitter<ChatItem | undefined | null | void>();

  /**
   * Public readonly property to expose the event for external listeners to detect tree data changes.
   */
  readonly onDidChangeTreeData: vscode.Event<
    ChatItem | undefined | null | void
  > = this._onDidChangeTreeData.event;

  /**
   * Method to refresh the tree view. It triggers a refresh of the tree data by firing an event
   * with undefined data, indicating a complete refresh.
   */
  refresh(): void {
    // Since the entire tree data might be updated after a change to the config,
    // we pass undefined to indicate that the whole tree has changed,
    // and it should refresh all its items.
    this._onDidChangeTreeData.fire(undefined);
  }

  /**
   * Method to retrieve a tree item given an element. It simply returns the passed tree item.
   *
   * @param element - The tree item to retrieve.
   * @returns The retrieved tree item.
   */
  getTreeItem(element: vscode.TreeItem) {
    return element;
  }

  /**
   * Method to dynamically fetch the children of a tree item based on the configuration file.
   * It returns different sets of children items depending on the type of the passed item
   * (database, API, or file).
   *
   * @param element - The parent tree item for which to retrieve children.
   * @returns A promise resolving to an array of child ChatItem objects.
   */
  getChildren(element?: ChatItem): Thenable<ChatItem[]> {
    if (vscode.workspace.workspaceFolders) {
      const root = vscode.workspace.workspaceFolders[0].uri.fsPath;

      // The root call for getChildren has no element
      if (!element) {
        const config = getConfigIfAny();

        if (config === null) {
          // If no config available then if means there are no interfaces...
          return Promise.resolve([]);
        }

        const chatItems = config.chats
          ? config.chats.map((chat) => {
              return new ChatItem(chat.id, chat.title);
            })
          : [];

        return Promise.resolve(chatItems);
      }
    }

    return Promise.resolve([]);
  }
}
