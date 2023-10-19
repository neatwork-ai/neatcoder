import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { getFilename, getOrCreateConfigPath } from "../utils/utils";
import { InterfacesProvider } from "../foreignInterfaces/providers";
import { AppStateManager } from "../core/appData";
import { logger } from "../utils/logger";

/**
 * Set up watchers for schemas specified in the configuration file.
 *
 * @param schemaWatchers - An object holding fs.FSWatcher instances associated with different paths.
 * @param interfacesProvider - Instance of InterfacesProvider to refresh interface views.
 * @param appManager - Instance of AppStateManager to manage the app's state.
 * @returns An array of fs.FSWatcher instances created during the setup process (if any).
 */
export function setupSchemaWatchers(
  schemaWatchers: { [key: string]: fs.FSWatcher },
  interfacesProvider: InterfacesProvider,
  appManager: AppStateManager
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
          appManager
        );
      }
    });
  }
}

/**
 * Set up a watcher for a specific interface.
 *
 * @param name - Name of the interface.
 * @param absolutePath - Absolute path to the interface schema.
 * @param interfacesProvider - Instance of InterfacesProvider to refresh interface views.
 * @param appManager - Instance of AppStateManager to manage the app's state.
 * @returns The fs.FSWatcher instance created for the specified interface.
 */
function setupWatcherForInterface(
  name: string,
  absolutePath: string,
  interfacesProvider: InterfacesProvider,
  appManager: AppStateManager
): fs.FSWatcher {
  logger.appendLine("[INFO] Setting up schema watcher for " + name);
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
            handleNewFile(name, fullPath, appManager);

            // Refresh the view
            interfacesProvider.refresh();
          } else {
            // File was deleted
            handleFileDelete(name, filename, appManager);

            // Refresh the view
            interfacesProvider.refresh();
          }
        } else if (
          event === "change" &&
          fs.existsSync(fullPath) &&
          fs.statSync(fullPath).isFile()
        ) {
          handleFileEdit(name, fullPath, appManager);
        }
      }
    }
  );

  return watcher;
}

/**
 * Handle the addition of a new file in the watched directory.
 *
 * @param interfaceName - Name of the interface associated with the new file.
 * @param filePath - Path to the new file.
 * @param appManager - Instance of AppStateManager to manage the app's state.
 */
function handleNewFile(
  interfaceName: string,
  filePath: string,
  appManager: AppStateManager
) {
  const schema = fs.readFileSync(filePath, "utf8");
  const schemaName = getFilename(filePath);

  appManager.addSchema(interfaceName, schemaName, schema);
  logger.appendLine(`[INFO] Adding Schema ${schemaName}`);
}

/**
 * Handle the modification of a file in the watched directory.
 *
 * @param interfaceName - Name of the interface associated with the modified file.
 * @param filePath - Path to the modified file.
 * @param appManager - Instance of AppStateManager to manage the app's state.
 */
function handleFileEdit(
  interfaceName: string,
  filePath: string,
  appManager: AppStateManager
) {
  const schema = fs.readFileSync(filePath, "utf8");
  const schemaName = getFilename(filePath);

  // It will replace the previous schema state
  appManager.addSchema(interfaceName, schemaName, schema);
  logger.appendLine(`[INFO] Editing Schema ${schemaName}`);
}

/**
 * Handle the deletion of a file in the watched directory.
 *
 * @param interfaceName - Name of the interface associated with the deleted file.
 * @param filename - Name of the deleted file.
 * @param appManager - Instance of AppStateManager to manage the app's state.
 */
function handleFileDelete(
  interfaceName: string,
  filename: string,
  appManager: AppStateManager
) {
  const schemaName = getFilename(filename);

  logger.appendLine(`[INFO] Removing Schema ${schemaName}`);
  appManager.removeSchema(interfaceName, schemaName);
}
