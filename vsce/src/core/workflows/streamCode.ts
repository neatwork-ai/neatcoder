import { Position, TextDocument, window } from "vscode";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { logger } from "../../utils/logger";

let currentPosition = new Position(0, 0);

/**
 * Asynchronously streams a token (typically a portion of code) to the active text document in the VS Code editor.
 * The function performs the following operations:
 * 1. Opens the active text document in the editor.
 * 2. Inserts the given token at the current position.
 * 3. Updates the current position to point to the end of the document.
 *
 * @param token - The string token to be streamed into the active text document.
 * @param activeTextDocument - The currently active text document in the VS Code editor.
 * @returns Promise<void> - A promise that resolves once the token has been streamed to the document.
 */
// export async function streamCode(
//   token: string,
//   activeTextDocument: TextDocument
// ): Promise<void> {
//   try {
//     const editor = await window.showTextDocument(activeTextDocument, {
//       preview: false,
//       preserveFocus: true,
//     });

//     await editor.edit((editBuilder) => {
//       editBuilder.insert(currentPosition, token); // Adding a space after every word
//     });

//     // Update currentPosition to point to the new end of the document
//     const docContent = editor.document.getText();
//     currentPosition = editor.document.positionAt(docContent.length);
//   } catch (error) {
//     console.log(error);
//     throw error;
//   }
// }

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
  } catch (error) {
    console.error(error);
    throw error;
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
        } catch (error) {
          logger.appendLine(`[ERROR] Could not read file: ${error}`);
        }
      }
    }
  };

  readDirRecursively(srcFolderPath);

  return record;
}
