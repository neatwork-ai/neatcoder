import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { InterfacesProvider } from "../providers/interfaces";
import { setupSchemaWatchers } from "./schemaWatcher";
import { getOrCreateConfigPath, getRoot, saveAppStateToFile } from "../utils";
import * as wasm from "../../pkg/neatcoder";
import { AppStateManager } from "../appStateManager";

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
  appManager: AppStateManager,
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
    if (filename) {
      const fullPath = path.join(root, ".neat", filename);
      if (fs.existsSync(fullPath) && fs.statSync(fullPath).isFile()) {
        logger.appendLine(`[INFO] Detected changes in: ${filename}`);

        // Refresh UI
        interfacesProvider.refresh();

        // Read the new content
        const newContentString = fs.readFileSync(fullPath, "utf-8");
        let newContent;

        try {
          newContent = JSON.parse(newContentString);
        } catch (error) {
          logger.appendLine(`[ERROR] Failed to parse JSON: ${error}`);
          return;
        }

        // Compare and handle additions
        const bool1 = handleAdditions(
          originalConfig.dbs,
          newContent.dbs,
          appManager,
          logger,
          createDbInterface
        );

        const bool2 = handleAdditions(
          originalConfig.apis,
          newContent.apis,
          appManager,
          logger,
          createApiInterface
        );

        // Compare and handle removals
        const bool3 = handleRemovals(
          originalConfig.dbs,
          newContent.dbs,
          appManager,
          logger
        );

        const bool4 = handleRemovals(
          originalConfig.apis,
          newContent.apis,
          appManager,
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
            appManager,
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
  appManager: AppStateManager,
  logger: vscode.OutputChannel,
  callback: (newItem: any, logger: vscode.OutputChannel) => any
): Boolean {
  const newItems = updated.filter(
    (item) => !original.some((origItem) => origItem.name === item.name)
  );

  const toUpdate = newItems.length > 0;
  for (const newItem of newItems) {
    const appInterface = callback(newItem, logger);
    appManager.addInterface(appInterface);
  }
  return toUpdate;
}

function handleRemovals(
  original: any[],
  updated: any[],
  appManager: AppStateManager,
  logger: vscode.OutputChannel
): Boolean {
  const removedItems = original.filter(
    (item) => !updated.some((updatedItem) => updatedItem.name === item.name)
  );

  const toUpdate = removedItems.length > 0;

  for (const removedItem of removedItems) {
    appManager.removeInterface(removedItem.name);

    logger.appendLine(`[INFO] Removing Interface ${removedItem.name}`);
  }
  return toUpdate;
}

function createDbInterface(
  newItem: any,
  logger: vscode.OutputChannel
): wasm.Interface {
  const dbType: wasm.DbType = newItem.dbType;
  const database: wasm.Database = new wasm.Database(newItem.name, dbType, {});

  logger.appendLine(`[INFO] Adding Database Interface ${newItem.name}`);

  const inter_ = wasm.Interface.newDb(database);

  return inter_;
}

function createApiInterface(
  newItem: any,
  logger: vscode.OutputChannel
): wasm.Interface {
  const apiType: wasm.ApiType = newItem.apiType;

  const api: wasm.Api = new wasm.Api(newItem.name, apiType, {});

  logger.appendLine(`[INFO] Adding Api Interface ${newItem.name}`);

  return wasm.Interface.newApi(api);
}
