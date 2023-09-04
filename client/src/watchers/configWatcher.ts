import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { InterfacesProvider } from "../providers/interfaces";
import { setupSchemaWatchers } from "./schemaWatcher";
import { getOrCreateConfigPath, getRoot } from "../utils";
import * as wasm from "../../pkg/neatcoder";

let originalConfig: any;

/**
 * Sets up a watcher for the `.neat/config.json` configuration file.
 * It handles the refresh of the Interfaces UI view and communicates changes to the TCP server.
 *
 * @param interfacesProvider - The provider for the Interfaces UI view.
 * @param logger - Output channel for logging events.
 * @param appState - A mutable reference to the application state
 * @returns The file watcher for the `.neat/config.json` file.
 */
export function setupConfigWatcher(
  schemaWatchers: { [key: string]: fs.FSWatcher },
  interfacesProvider: InterfacesProvider,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
) {
  if (!vscode.workspace.workspaceFolders) {
    return;
  }

  const root = getRoot();
  const configPath = getOrCreateConfigPath();

  // Read and store the original content
  if (fs.existsSync(configPath)) {
    const content = fs.readFileSync(configPath, "utf-8");
    originalConfig = JSON.parse(content);
  } else {
    originalConfig = { dbs: [], apis: [] };
  }

  // Watch the config.json
  return fs.watch(configPath, (event, filename) => {
    logger.appendLine(`[INFO] Some event:${event}`);
    if (filename) {
      const fullPath = path.join(root, ".neat", filename);
      logger.appendLine(`[INFO] FULL PATH:${fullPath}`);
      logger.appendLine(`[INFO] FULL PATH EXISTS? ${fs.existsSync(fullPath)}`);
      logger.appendLine(`[INFO] FILE SYNC:${fs.statSync(fullPath).isFile()}`);
      if (fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
        logger.appendLine(`[INFO] Detected changes in: ${filename}`);

        // Refresh UI
        interfacesProvider.refresh();
        logger.appendLine(`a`);

        // Refresh Server
        // Read the new content
        const newContentString = fs.readFileSync(fullPath, "utf-8");
        logger.appendLine(`b`);
        logger.appendLine(`[INFO] THE NEW CONFIG :${newContentString}`);
        const newContent = JSON.parse(newContentString);
        logger.appendLine(`c`);

        // Compare and handle additions
        const bool1 = handleAdditions(
          originalConfig.dbs,
          newContent.dbs,
          appState,
          logger,
          createDbInterface
        );

        const bool2 = handleAdditions(
          originalConfig.apis,
          newContent.apis,
          appState,
          logger,
          createApiInterface
        );

        // Compare and handle removals
        const bool3 = handleRemovals(
          originalConfig.dbs,
          newContent.dbs,
          appState,
          logger
        );
        const bool4 = handleRemovals(
          originalConfig.apis,
          newContent.apis,
          appState,
          logger
        );

        const toUpdate = bool1 || bool2 || bool3 || bool4;

        if (toUpdate) {
          // Close the old schema watchers
          for (const key in schemaWatchers) {
            if (schemaWatchers.hasOwnProperty(key)) {
              schemaWatchers[key].close();
            }
          }

          // Set up new schema watchers based on the updated config.json
          setupSchemaWatchers(
            schemaWatchers,
            interfacesProvider,
            appState,
            logger
          );
        }

        // Update original content
        originalConfig = newContent;
      }
    }
  });
}

function handleAdditions(
  original: any[],
  updated: any[],
  appState: wasm.AppState,
  logger: vscode.OutputChannel,
  callback: (newItem: any, logger: vscode.OutputChannel) => any
): Boolean {
  const newItems = updated.filter(
    (item) => !original.some((origItem) => origItem.name === item.name)
  );

  const toUpdate = newItems.length > 0;

  for (const newItem of newItems) {
    const appInterface = callback(newItem, logger);
    appState.addInterface(appInterface);
  }
  return toUpdate;
}

function handleRemovals(
  original: any[],
  updated: any[],
  appState: wasm.AppState,
  logger: vscode.OutputChannel
): Boolean {
  const removedItems = original.filter(
    (item) => !updated.some((updatedItem) => updatedItem.name === item.name)
  );

  const toUpdate = removedItems.length > 0;

  for (const removedItem of removedItems) {
    appState.removeInterface(removedItem.name);

    logger.appendLine(`[INFO] Removing Interface ${removedItem.name}`);
  }
  return toUpdate;
}

function createDbInterface(
  newItem: any,
  logger: vscode.OutputChannel
): wasm.Interface {
  const dbType: wasm.DbType = wasm.dbTypeFromFriendlyUX(newItem.dbType);

  const database: wasm.Database = new wasm.Database(
    newItem.name,
    dbType,
    // newItem.port, // TODO
    // newItem.host,
    {}
  );

  logger.appendLine(`[INFO] Adding Database Interface ${newItem.name}`);

  return wasm.Interface.newDb(database);
}

function createApiInterface(
  newItem: any,
  logger: vscode.OutputChannel
): wasm.Interface {
  const apiType: wasm.ApiType = wasm.apiTypeFromFriendlyUX(newItem.apiType);

  const api: wasm.Api = new wasm.Api(
    newItem.name,
    apiType,
    newItem.port,
    newItem.host,
    {}
  );

  logger.appendLine(`[INFO] Adding Api Interface ${newItem.name}`);

  return wasm.Interface.newApi(api);
}
