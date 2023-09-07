import * as net from "net";
import * as vscode from "vscode";
import { EventEmitter } from "events";
import { OutputChannel, window, workspace } from "vscode";
import { writeHeapSnapshot } from "v8";

/**
 * Creates a file in the root of the current workspace.
 * @param fileName - The name of the file to create.
 */
export async function createFile(
  fileName: string,
  logger: vscode.OutputChannel,
): Promise<void> {
  // Check if there are workspace folders
  if (!workspace.workspaceFolders || workspace.workspaceFolders.length === 0) {
    logger.appendLine("[ERROR] No workspace folder found. Cannot create file.");
    return;
  }

  // Get the root workspace folder
  const workspaceRoot = workspace.workspaceFolders[0].uri;

  // Create the file URI
  const fileUri = workspaceRoot.with({
    path: `${workspaceRoot.path}/${fileName}`,
  });

  try {
    // Create an empty file
    await workspace.fs.writeFile(fileUri, new Uint8Array());

    // Show success message
    window.showInformationMessage(`File '${fileName}' created successfully.`);
  } catch (err) {
    // Log error message
    logger.appendLine(`[ERROR] Failed to create file '${fileName}': ${err}`);

    // Show error message
    window.showErrorMessage(`Failed to create file '${fileName}'`);
  }
}
