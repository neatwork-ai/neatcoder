import * as vscode from "vscode";

/**
 * Creates a file in the root of the current workspace.
 * @param fileName - The name of the file to create.
 */
export async function createFile(
  fileName: string,
  logger: vscode.OutputChannel
): Promise<void> {
  // Check if there are workspace folders
  if (
    !vscode.workspace.workspaceFolders ||
    vscode.workspace.workspaceFolders.length === 0
  ) {
    logger.appendLine("[ERROR] No workspace folder found. Cannot create file.");
    return;
  }

  // Get the root workspace folder
  const workspaceRoot = vscode.workspace.workspaceFolders[0].uri;

  // Create the file URI
  const fileUri = workspaceRoot.with({
    path: `${workspaceRoot.path}/${fileName}`,
  });

  try {
    // Create an empty file
    await vscode.workspace.fs.writeFile(fileUri, new Uint8Array());
  } catch (err) {
    // Log error message
    logger.appendLine(`[ERROR] Failed to create file '${fileName}': ${err}`);

    // Show error message
    vscode.window.showErrorMessage(`Failed to create file '${fileName}'`);
  }
}
