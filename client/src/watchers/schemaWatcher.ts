import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { getFilename, getOrCreateConfigPath } from "../utils";
import { InterfacesProvider } from "../providers/interfaces";
import * as wasm from "../../pkg/code_builder";

/**
 * Sets up watchers for schema directories defined in a configuration file.
 * Sends changes (addition, modification) to a TCP server.
 *
 * @param appState - A mutable reference to the application state
 * @param logger - Output channel for logging events.
 */
export function setupSchemaWatchers(
  schemaWatchers: { [key: string]: fs.FSWatcher },
  interfacesProvider: InterfacesProvider,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
) {
  if (!vscode.workspace.workspaceFolders) {
    return [];
  }

  const root = vscode.workspace.workspaceFolders[0].uri.fsPath;
  const configPath = getOrCreateConfigPath();

  const configContent = fs.readFileSync(configPath, "utf-8");
  const config: { paths: { name: string; path: string }[] } =
    JSON.parse(configContent);

  if (config && config.paths) {
    config.paths.forEach(({ name: name, path: relPath }) => {
      const absolutePath = path.join(root, relPath);

      // Close an existing watcher if it exists for the path
      if (schemaWatchers[absolutePath]) {
        schemaWatchers[absolutePath].close();
      }

      // Check if the directory exists before setting up the watcher
      if (
        fs.existsSync(absolutePath) &&
        fs.statSync(absolutePath).isDirectory()
      ) {
        schemaWatchers[absolutePath] = setupWatcherForInterface(
          name,
          absolutePath,
          interfacesProvider,
          appState,
          logger
        );
      }
    });
  }
}

function setupWatcherForInterface(
  name: string,
  absolutePath: string,
  interfacesProvider: InterfacesProvider,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
): fs.FSWatcher {
  // Your existing watcher setup logic here, but return the watcher
  const watcher = fs.watch(
    absolutePath,
    { recursive: true },
    (event, filename) => {
      if (filename) {
        const fullPath = path.join(absolutePath, filename);

        if (event === "rename") {
          if (fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
            // File was added or renamed
            handleNewFile(name, fullPath, appState, logger);

            // Refresh the view
            interfacesProvider.refresh();
          } else {
            // File was deleted
            handleFileDelete(name, filename, appState, logger);

            // Refresh the view
            interfacesProvider.refresh();
          }
        } else if (
          event === "change" &&
          fs.existsSync(fullPath) &&
          fs.statSync(fullPath).isFile()
        ) {
          handleFileEdit(name, fullPath, appState, logger);
        }
      }
    }
  );

  return watcher;
}

function handleNewFile(
  interfaceName: string,
  filePath: string,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
) {
  const schema = fs.readFileSync(filePath, "utf8");
  const schemaName = getFilename(filePath);

  appState.addSchema(interfaceName, schemaName, schema);
  logger.appendLine(`[INFO] Adding Schema ${schemaName}`);
}

/**
 * Handles the creation of a new schema file by sending its details to the TCP server.
 *
 * @param interfaceName - The name of the interface for the schema.
 * @param filePath - Absolute path to the new schema file.
 * @param appState - A mutable reference to the application state
 */
function handleFileEdit(
  interfaceName: string,
  filePath: string,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
) {
  const schema = fs.readFileSync(filePath, "utf8");
  const schemaName = getFilename(filePath);

  // It will replace the previous schema state
  appState.addSchema(interfaceName, schemaName, schema);
  logger.appendLine(`[INFO] Editing Schema ${schemaName}`);
}

function handleFileDelete(
  interfaceName: string,
  filename: string,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
) {
  const schemaName = getFilename(filename);

  logger.appendLine(`[INFO] TCP: Removing Schema ${schemaName}`);
  appState.removeSchema(interfaceName, schemaName);
}
