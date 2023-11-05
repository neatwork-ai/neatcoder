import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import InterfaceItem from "../models";
import { getOrCreateConfigPath } from "../../utils/utils";

/**
 * Opens a file dialog to allow the user to select schema files.
 * The selected files are then copied to the destination directory specific to the provided database interface item.
 *
 * @param {InterfaceItem} item - The database interface item.
 * @returns {Promise<void>}
 */
export async function addSchema(item: InterfaceItem): Promise<void> {
  const options: vscode.OpenDialogOptions = {
    canSelectMany: true, // Allow the selection of multiple files
    openLabel: "Open", // The label for the open button
    filters: {},
  };

  // Show the dialog
  const fileUris = await vscode.window.showOpenDialog(options);

  if (fileUris && fileUris.length) {
    const root = vscode.workspace.workspaceFolders
      ? vscode.workspace.workspaceFolders[0].uri.fsPath
      : "";
    const destinationPath = getDestinationPath(item.label as string, root);

    if (!destinationPath) {
      vscode.window.showErrorMessage(
        `Path not found for database: ${item.label}`
      );
      return;
    }

    await copyFilesToDestination(fileUris, path.join(root, destinationPath));
  }
}

/**
 * Determines the destination path for the provided database name.
 * The path is retrieved from the configuration file.
 *
 * @param {string} dbName - The name of the database.
 * @param {string} root - The root path of the workspace.
 * @returns {string | null} - The destination path if found; null otherwise.
 */
function getDestinationPath(dbName: string, root: string): string | null {
  const configPath = getOrCreateConfigPath();
  const configContent = fs.readFileSync(configPath, "utf-8");
  const config = JSON.parse(configContent);

  const targetPathObj = config.paths.find((p: any) => p.name === dbName);
  return targetPathObj ? targetPathObj.path : null;
}

/**
 * Copies the provided files to the specified destination directory.
 *
 * @param {vscode.Uri[]} files - The files to copy.
 * @param {string} destination - The destination directory.
 */
async function copyFilesToDestination(
  files: vscode.Uri[],
  destination: string
) {
  for (const file of files) {
    const destFilePath = path.join(destination, path.basename(file.fsPath));
    fs.copyFile(file.fsPath, destFilePath, (err) => {
      if (err) {
        vscode.window.showErrorMessage(`Failed to copy file: ${err.message}`);
      }
    });
  }
}
