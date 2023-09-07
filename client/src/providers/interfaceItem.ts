import * as vscode from "vscode";
import * as path from "path";

export enum ItemType {
  database,
  api,
  file,
}

export class InterfaceItem extends vscode.TreeItem {
  public type: ItemType;

  constructor(
    public readonly label: string,
    public readonly description: string, // if empty then it's a file
    public readonly parent: string | null,
    collapsibleState: vscode.TreeItemCollapsibleState,
    type: ItemType
  ) {
    super(label, collapsibleState);

    this.type = type;
    this.parent = parent;
    // Set the description to the interface type
    this.description = description;

    if (this.type === ItemType.database) {
      this.contextValue = "databaseItem";
      // Add icon for database if needed
      this.iconPath = path.join(__filename, "..", "..", "assets", "db-2.svg");
    } else if (this.type === ItemType.api) {
      this.contextValue = "apiItem";
      this.iconPath = path.join(__filename, "..", "..", "assets", "api-2.svg");
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
