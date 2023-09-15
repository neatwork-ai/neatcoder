import { Position, TextDocument, window } from "vscode";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";

let currentPosition = new Position(0, 0);

export async function streamCode(
  token: string,
  activeTextDocument: TextDocument,
  logger: vscode.OutputChannel
): Promise<void> {
  try {
    const editor = await window.showTextDocument(activeTextDocument);

    await editor.edit((editBuilder) => {
      editBuilder.insert(currentPosition, token); // Adding a space after every word
    });

    // Update currentPosition to point to the new end of the document
    const docContent = editor.document.getText();
    currentPosition = editor.document.positionAt(docContent.length);
  } catch (error) {
    console.log(error);
    throw error;
  }
}

export async function scanSourceFolder(
  logger: vscode.OutputChannel
): Promise<Record<string, string>> {
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

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
