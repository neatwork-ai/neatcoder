import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import InterfaceItem from "../providers/interfaceItem";
import { getOrCreateConfigPath, getRoot } from "../utils";

export function removeSchema(item: InterfaceItem) {
  if (!item) {
    vscode.window.showErrorMessage("Please select a schema to remove.");
    return;
  }

  const schemaName = item.label; // Extracting schema name from the clicked item

  const configPath = getOrCreateConfigPath();

  const configContent = fs.readFileSync(configPath, "utf-8");
  const config = JSON.parse(configContent);

  // Assuming the parent of the schema is an Interface (either DB or API), and is selected when you right-click on the schema.
  const parentInterfaceName = item.parent;
  if (!parentInterfaceName) {
    vscode.window.showErrorMessage(
      `Could not identify parent interface for schema ${schemaName}.`
    );
    return;
  }

  const interfacePathObj = config.paths.find(
    (entry: any) => entry.name === parentInterfaceName
  );
  if (!interfacePathObj) {
    vscode.window.showErrorMessage(
      `Interface named ${parentInterfaceName} not found in config.`
    );
    return;
  }

  // Construct the path to the schema file
  const root = getRoot();
  const schemaFilePath = path.join(root, interfacePathObj.path, schemaName);

  if (!fs.existsSync(schemaFilePath)) {
    vscode.window.showErrorMessage(
      `Schema file not found at ${schemaFilePath}`
    );
    return;
  }

  // Delete the schema file
  fs.unlinkSync(schemaFilePath);

  vscode.window.showInformationMessage(
    `Schema named ${schemaName} removed successfully.`
  );
}
