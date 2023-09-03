import * as vscode from "vscode";
import * as path from "path";
import * as fs from "fs";
import * as wasm from "../../pkg/neatcoder";

/**
 * Sets up a FileSystemWatcher for the `src` folder in the workspace and
 * sends changes (addition, modification, deletion) to a TCP server.
 *
 * @param tcpClient - The client used to communicate with the TCP server.
 * @param logger - Output channel for logging events.
 * @returns A FileSystemWatcher instance that watches for changes in the `src` folder.
 */
export function setupSrcFolderWatcher(logger: vscode.OutputChannel) {
  if (!vscode.workspace.workspaceFolders) {
    return;
  }

  const root = vscode.workspace.workspaceFolders[0].uri.fsPath;
  const srcPath = path.join(root, "src");

  // Create the FileSystemWatcher for the `src` folder
  const fileWatcher = vscode.workspace.createFileSystemWatcher("src/**/*.*");

  // React to file changes
  fileWatcher.onDidChange((uri) => {
    logger.appendLine(`File changed in src folder: ${uri.fsPath}`);
    const fileContent = fs.readFileSync(uri.fsPath, "utf8");

    // TODO: Rethink this watchetr
    // const command: ClientCommand = {
    //   addSourceFile: {
    //     filename: uri.fsPath,
    //     file: fileContent,
    //   },
    // };
    // // Send the changes to the TCP Server
    // logger.appendLine(`[INFO] TCP: Editing File:${uri.fsPath}`);
    // tcpClient.send(command);
  });

  fileWatcher.onDidCreate((uri) => {
    const fileContent = fs.readFileSync(uri.fsPath, "utf8");

    // const command: ClientCommand = {
    //   addSourceFile: {
    //     filename: uri.fsPath,
    //     file: fileContent,
    //   },
    // };
    // // Send the creation event to the TCP Server
    // logger.appendLine(`[INFO] TCP: Adding File:${uri.fsPath}`);
    // tcpClient.send(command);
  });

  fileWatcher.onDidDelete((uri) => {
    // const command: ClientCommand = {
    //   removeSourceFile: {
    //     filename: uri.fsPath,
    //   },
    // };
    // // Send the deletion event to the TCP Server
    // logger.appendLine(`[INFO] TCP: Removing File:${uri.fsPath}`);
    // tcpClient.send(command);
  });

  // Return the watcher so it can be added to context.subscriptions in the activate function
  return fileWatcher;
}
