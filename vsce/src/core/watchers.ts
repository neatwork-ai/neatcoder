import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import { getOrCreateConfigPath, getRoot } from "../utils/utils";
import * as wasm from "../../pkg/neatcoder";
import { appDataManager } from "../core/appData";
import { logger } from "../utils/logger";
import { InterfacesProvider } from "../foreignInterfaces/providers";
import { setupSchemaWatchers } from "../foreignInterfaces/watchers";

let originalConfig: any;

export function setupDotNeatWatcher(
  schemaWatchers: { [key: string]: fs.FSWatcher },
  interfacesProvider: InterfacesProvider,
  appManager: appDataManager
) {
  if (!vscode.workspace.workspaceFolders) {
    return;
  }

  const root = getRoot();
  const configDir = path.join(root, ".neat");
  const configPath = path.join(configDir, "config.json");

  if (fs.existsSync(configDir)) {
    if (fs.existsSync(configPath)) {
      setupFileWatcher(schemaWatchers, interfacesProvider, appManager);
      setupSchemaWatchers(schemaWatchers, interfacesProvider, appManager);
    } else {
      watchForFileCreation(configDir, "config.json", () => {
        setupFileWatcher(schemaWatchers, interfacesProvider, appManager);
        setupSchemaWatchers(schemaWatchers, interfacesProvider, appManager);
      });
    }
  } else {
    watchForFileCreation(root, ".neat", () => {
      watchForFileCreation(configDir, "config.json", () => {
        setupFileWatcher(schemaWatchers, interfacesProvider, appManager);
        setupSchemaWatchers(schemaWatchers, interfacesProvider, appManager);
      });
    });
  }
}

function watchForFileCreation(
  directory: string,
  targetFile: string,
  callback: () => void
) {
  const watcher = fs.watch(directory, (event, filename) => {
    if (filename === targetFile && event === "rename") {
      watcher.close();
      callback();
    }
  });
}

/**
 * Sets up a watcher for the configuration file to handle changes in the file, and
 * synchronises the AppData via the appManager
 *
 * @param schemaWatchers - The watchers for schema changes.
 * @param interfacesProvider - Provider for managing interfaces.
 * @param appManager - The application manager for handling state and configurations.
 */
export function setupFileWatcher(
  schemaWatchers: { [key: string]: fs.FSWatcher },
  interfacesProvider: InterfacesProvider,
  appManager: appDataManager
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
          createDbInterface
        );

        const bool2 = handleAdditions(
          originalConfig.apis,
          newContent.apis,
          appManager,
          createApiInterface
        );

        // Compare and handle removals
        const bool3 = handleRemovals(
          originalConfig.dbs,
          newContent.dbs,
          appManager
        );

        const bool4 = handleRemovals(
          originalConfig.apis,
          newContent.apis,
          appManager
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
          setupSchemaWatchers(schemaWatchers, interfacesProvider, appManager);
        }

        // Update original content
        originalConfig = newContent;
      }
    }
  });
}

/**
 * Handles the addition of new items in the configuration.
 *
 * @param original - The original array of items.
 * @param updated - The updated array of items.
 * @param appManager - The application manager for handling state and configurations.
 * @param callback - Callback function to create a new interface for the added item.
 * @returns Boolean indicating if there were any new items added.
 */
function handleAdditions(
  original: any[],
  updated: any[],
  appManager: appDataManager,
  callback: (newItem: any) => any
): Boolean {
  const newItems = updated.filter(
    (item) => !original.some((origItem) => origItem.name === item.name)
  );

  const toUpdate = newItems.length > 0;
  for (const newItem of newItems) {
    const appInterface = callback(newItem);
    appManager.addInterface(appInterface);
  }
  return toUpdate;
}

/**
 * Handles the removal of items from the configuration.
 *
 * @param original - The original array of items.
 * @param updated - The updated array of items.
 * @param appManager - The application manager for handling state and configurations.
 * @returns Boolean indicating if there were any items removed.
 */
function handleRemovals(
  original: any[],
  updated: any[],
  appManager: appDataManager
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

/**
 * Creates a new database interface.
 *
 * @param newItem - The new database item.
 * @returns A new database interface.
 */
function createDbInterface(newItem: any): wasm.Interface {
  const dbType: wasm.DbType = newItem.dbType;
  const database: wasm.Database = new wasm.Database(newItem.name, dbType, {});

  logger.appendLine(`[INFO] Adding Database Interface ${newItem.name}`);

  const inter_ = wasm.Interface.newDb(database);

  return inter_;
}

/**
 * Creates a new API interface.
 *
 * @param newItem - The new API item.
 * @returns A new API interface.
 */
function createApiInterface(newItem: any): wasm.Interface {
  const apiType: wasm.ApiType = newItem.apiType;

  const api: wasm.Api = new wasm.Api(newItem.name, apiType, {});

  logger.appendLine(`[INFO] Adding Api Interface ${newItem.name}`);

  return wasm.Interface.newApi(api);
}
