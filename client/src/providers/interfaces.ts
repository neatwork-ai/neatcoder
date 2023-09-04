import * as vscode from "vscode";
import * as path from "path";
import * as fs from "fs";
import { InterfaceItem, ItemType } from "./interfaceItem";
import { getOrCreateConfigPath } from "../utils";
import * as wasm from "../../pkg/neatcoder";

export type DbEntry = {
  name: string;
  dbType: string;
};

export type ApiEntry = {
  name: string;
  apiType: string;
};

export class InterfacesProvider
  implements vscode.TreeDataProvider<InterfaceItem>
{
  constructor() {
    this._onDidChangeTreeData = new vscode.EventEmitter();
    this.onDidChangeTreeData = this._onDidChangeTreeData.event;
  }

  // Explicitly declare the property with its type
  private _onDidChangeTreeData: vscode.EventEmitter<
    InterfaceItem | undefined | null | void
  > = new vscode.EventEmitter<InterfaceItem | undefined | null | void>();
  readonly onDidChangeTreeData: vscode.Event<
    InterfaceItem | undefined | null | void
  > = this._onDidChangeTreeData.event;

  refresh(): void {
    // Since the entire tree data might be updated after a change to the config,
    // we pass undefined to indicate that the whole tree has changed,
    // and it should refresh all its items.
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: vscode.TreeItem) {
    return element;
  }

  getChildren(element?: InterfaceItem): Thenable<InterfaceItem[]> {
    if (vscode.workspace.workspaceFolders) {
      const root = vscode.workspace.workspaceFolders[0].uri.fsPath;

      if (!element) {
        // If no element provided, return the database and API list
        const configPath = getOrCreateConfigPath();
        const configContent = fs.readFileSync(configPath, "utf-8");
        const config: { dbs: DbEntry[]; apis: ApiEntry[] } =
          JSON.parse(configContent); // Assuming your config has an 'apis' array similar to 'dbs'

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
