import { Position, TextDocument, window } from "vscode";
import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";

export async function streamCode(
  token: string,
  activeTextDocument: TextDocument
): Promise<void> {
  // Here, we append the word to the opened document
  // TODO: No need to continuously open the text document...
  const editor = await window.showTextDocument(activeTextDocument);
  const lastLine = activeTextDocument.lineAt(activeTextDocument.lineCount - 1);
  const position = new Position(lastLine.lineNumber, lastLine.text.length);
  editor.edit((editBuilder) => {
    editBuilder.insert(position, token); // Adding a space after every word
  });
}

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

  const files = fs.readdirSync(srcFolderPath);
  const record: Record<string, string> = {};

  for (const file of files) {
    const filePath = path.join(srcFolderPath, file);
    const fileContent = fs.readFileSync(filePath, "utf-8");
    record[file] = fileContent;
  }

  return record;
}
