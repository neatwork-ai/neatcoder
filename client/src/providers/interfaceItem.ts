import * as vscode from "vscode";

export enum ItemType {
  database,
  api,
  file,
}

export class InterfaceItem extends vscode.TreeItem {
  public type: ItemType;

  constructor(
    public readonly label: string,
    public readonly description: string,
    public readonly parent: string | null,
    collapsibleState: vscode.TreeItemCollapsibleState,
    type: ItemType
  ) {
    super(label, collapsibleState);

    this.type = type;
    this.parent = parent;

    if (this.type === ItemType.database) {
      this.contextValue = "databaseItem";
      // Add icon for database if needed
      // this.iconPath = { ... };
    } else if (this.type === ItemType.api) {
      this.contextValue = "apiItem";
      // Add icon for file if needed
      // this.iconPath = { ... };
    } else if (this.type === ItemType.file) {
      this.contextValue = "fileItem";
      // Add icon for file if needed
      // this.iconPath = { ... };
    }
  }
}

export default InterfaceItem; // Export it if you want to use it in other modules.
