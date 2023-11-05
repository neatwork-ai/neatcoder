import { Position, TextDocument, window } from "vscode";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { logger } from "../../utils/logger";

let currentPosition = new Position(0, 0);

export async function streamCode(
  token: string,
  activeTextDocument: TextDocument
): Promise<void> {
  try {
    // Create a WorkspaceEdit instance to represent the edit
    const workspaceEdit = new vscode.WorkspaceEdit();

    // Update currentPosition to point to the new end of the document
    const docContent = activeTextDocument.getText();
    currentPosition = activeTextDocument.positionAt(docContent.length);

    // Insert the token at the current position
    workspaceEdit.insert(activeTextDocument.uri, currentPosition, token);

    // Apply the edit to the document
    await vscode.workspace.applyEdit(workspaceEdit);
  } catch (err) {
    console.error(`Error while sgreaming code to editor: ${err}`);
    window.showErrorMessage(`Error while sgreaming code to editor: ${err}`);
    throw new Error((err as Error).message);
  }
}

/**
 * Scans the source folder in the current workspace and returns a record containing file names as keys and their contents as values.
 * If the workspace is not open or the 'src' folder does not exist, an appropriate error message is displayed to the user.
 *
 * @returns Promise<Record<string, string>> - A promise that resolves to a record with file names as keys and their content as values.
 */
export async function scanSourceFolder(): Promise<Record<string, string>> {
  const workspaceFolders = vscode.workspace.workspaceFolders;

  if (!workspaceFolders) {
    vscode.window.showErrorMessage("No workspace is currently open.");
    return {};
  }

  const srcFolderPath = path.join(workspaceFolders[0].uri.fsPath, "src");

  if (!fs.existsSync(srcFolderPath)) {
    vscode.window.showErrorMessage(
      `'src' folder does not exist in the current workspace.`
    );
    return {};
  }

  logger.appendLine(`[INFO] reading source folder...`);
  const record: Record<string, string> = {};

  const readDirRecursively = (dirPath: string) => {
    const files = fs.readdirSync(dirPath);
    for (const file of files) {
      const filePath = path.join(dirPath, file);
      if (fs.lstatSync(filePath).isDirectory()) {
        readDirRecursively(filePath);
      } else {
        try {
          const fileContent = fs.readFileSync(filePath, "utf-8");
          record[file] = fileContent;
        } catch (err) {
          logger.appendLine(`[ERROR] Could not read file: ${err}`);
          console.error(err);
          throw new Error((err as Error).message);
        }
      }
    }
  };

  readDirRecursively(srcFolderPath);

  return record;
}
