import { Position, TextDocument, window } from "vscode";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";

let editing = false;

export async function streamCode(
  token: string,
  activeTextDocument: TextDocument,
  logger: vscode.OutputChannel
): Promise<void> {
  try {
    while (editing) {
      await new Promise((resolve) => setTimeout(resolve, 0));
    }

    editing = true;

    const editor = await window.showTextDocument(activeTextDocument);
    const lastLine = activeTextDocument.lineAt(
      activeTextDocument.lineCount - 1
    );
    const position = new Position(lastLine.lineNumber, lastLine.text.length);
    await editor.edit((editBuilder) => {
      editBuilder.insert(position, token); // Adding a space after every word
    });

    editing = false;
  } catch (error) {
    editing = false;
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
