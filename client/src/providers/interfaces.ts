import * as vscode from "vscode";
import * as path from "path";
import * as fs from "fs";
import { InterfaceItem, ItemType } from "../models/interfaceItem";
import { getConfigIfAny } from "../utils";
import * as wasm from "../../pkg/neatcoder";

/**
 * Type definition representing a database entry.
 */
export type DbEntry = {
  name: string;
  dbType: string;
};

/**
 * Type definition representing a database entry.
 */
export type PathEntry = {
  name: string;
  path: string;
};

/**
 * Type definition representing an API entry.
 */
export type ApiEntry = {
  name: string;
  apiType: string;
};

/**
 * Class implementing the vscode.TreeDataProvider interface to provide data for a tree view
 * in a VS Code extension. It manages the representation of database and API interfaces.
 */
export class InterfacesProvider
  implements vscode.TreeDataProvider<InterfaceItem>
{
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
    InterfaceItem | undefined | null | void
  > = new vscode.EventEmitter<InterfaceItem | undefined | null | void>();

  /**
   * Public readonly property to expose the event for external listeners to detect tree data changes.
   */
  readonly onDidChangeTreeData: vscode.Event<
    InterfaceItem | undefined | null | void
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
   * @returns A promise resolving to an array of child InterfaceItem objects.
   */
  getChildren(element?: InterfaceItem): Thenable<InterfaceItem[]> {
    if (vscode.workspace.workspaceFolders) {
      const root = vscode.workspace.workspaceFolders[0].uri.fsPath;

      // The root call for getChildren has no element
      if (!element) {
        const config = getConfigIfAny();

        if (config === null) {
          // If no config available then if means there are no interfaces...
          return Promise.resolve([]);
        }

        const dbItems = config.dbs
          ? config.dbs.map((db) => {
              return new InterfaceItem(
                db.name,
                wasm.dbTypeToFriendlyUX(Number(db.dbType)),
                null,
                vscode.TreeItemCollapsibleState.Collapsed,
                ItemType.database
              );
            })
          : [];

        const apiItems = config.apis
          ? config.apis.map((api) => {
              return new InterfaceItem(
                api.name,
                wasm.apiTypeToFriendlyUX(Number(api.apiType)),
                null,
                vscode.TreeItemCollapsibleState.Collapsed,
                ItemType.api
              );
            })
          : [];

        return Promise.resolve([...dbItems, ...apiItems]);
      } else if (element.type === ItemType.database) {
        // If an InterfaceItem representing a database is provided, return its files
        const interfaceFolderPath = path.join(
          root,
          ".neat",
          "dbs",
          element.label
        );
        const files = fs.readdirSync(interfaceFolderPath);

        return Promise.resolve(
          files.map((file) => {
            return new InterfaceItem(
              file,
              "", // No description for files, you can add if needed
              element.label,
              vscode.TreeItemCollapsibleState.None,
              ItemType.file
            );
          })
        );
      } else if (element.type === ItemType.api) {
        // If an InterfaceItem representing an API is provided, return its files
        const interfaceFolderPath = path.join(
          root,
          ".neat",
          "apis",
          element.label
        );
        const files = fs.readdirSync(interfaceFolderPath);

        return Promise.resolve(
          files.map((file) => {
            return new InterfaceItem(
              file,
              "", // No description for files, you can add if needed
              element.label,
              vscode.TreeItemCollapsibleState.None,
              ItemType.file
            );
          })
        );
      }
    }

    return Promise.resolve([]);
  }
}
