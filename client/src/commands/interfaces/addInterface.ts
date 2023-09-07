import * as vscode from "vscode";
import * as fs from "fs";
import { getOrCreateConfigPath, getOrCreateSchemasPath } from "../../utils";
import { apiList, dbList } from "../../models/mappings";
import * as wasm from "../../../pkg/neatcoder";
import { InterfacesProvider } from "../../providers/interfaces";

/**
 * Presents a quick pick list to the user to select or input a datastore type.
 * Handles the user's selection/input and updates the appropriate configuration.
 *
 * @returns {void}
 */
export function addInterface(
  interfaceType: wasm.InterfaceType,
  interfacesProvider: InterfacesProvider,
  logger: vscode.OutputChannel,
): void {
  {
    const list = getList(interfaceType);
    const placeholderName = getPlaceholderName(interfaceType);

    const quickPick = vscode.window.createQuickPick();
    quickPick.items = list.map((label) => ({ label }));

    quickPick.placeholder = `Select or type a ${placeholderName} type;`;
    quickPick.matchOnDescription = true;
    quickPick.matchOnDetail = true;

    let customEntered = false;

    // If the user types in the QuickPick box and the value doesn't match an item, show a "(custom)" option
    quickPick.onDidChangeValue((value) => {
      if (list.includes(value)) {
        customEntered = false;
        quickPick.items = list.map((item) => ({ label: item }));
      } else {
        customEntered = true;
        quickPick.items = [
          ...list.map((item) => ({ label: item })),
          { label: `${value} (custom)` },
        ];
      }
    });

    // Runs once the user proceeds by click enter or left-click with the mouse
    quickPick.onDidAccept(() => {
      let selectedType: string;

      if (
        quickPick.selectedItems[0] &&
        list.includes(quickPick.selectedItems[0].label)
      ) {
        customEntered = false;
        selectedType = quickPick.selectedItems[0].label;
      } else {
        selectedType = customEntered
          ? quickPick.value
          : quickPick.selectedItems[0].label;
      }

      // If the selected item has "(custom)", trim that part out
      if (selectedType.endsWith(" (custom)")) {
        selectedType = selectedType.substring(
          0,
          selectedType.lastIndexOf(" (custom)"),
        );
      }

      // Convert the string into DbType
      logger.appendLine(`[INFO] Selected Datastore type ${selectedType}`);
      handleSelection(
        placeholderName,
        interfaceType,
        selectedType,
        interfacesProvider,
        logger,
      );
      quickPick.dispose();
    });

    quickPick.show();
  }
}

/**
 * Handles the user's datastore type selection by prompting for additional inputs.
 * Updates the configuration file with the provided information.
 *
 * @param {DbType} dbType - The type of database selected by the user.
 * @returns {void}
 */
async function handleSelection(
  placeholderName: string,
  interfaceType: wasm.InterfaceType,
  selectedType: string,
  interfacesProvider: InterfacesProvider,
  logger: vscode.OutputChannel,
): Promise<void> {
  try {
    const interfaceName = await vscode.window.showInputBox({
      prompt: `Enter the name of the ${placeholderName}`,
      placeHolder: `${placeholderName} name`,
    });
    if (interfaceName === undefined) {
      return; // User canceled the input box
    }

    if (!interfaceName) {
      vscode.window.showErrorMessage("Datastore name cannot be empty!");
      return;
    }

    // Persist state
    const configPath = getOrCreateConfigPath();

    const content = await vscode.workspace.fs.readFile(
      vscode.Uri.file(configPath),
    );
    let config = JSON.parse(content.toString());

    // Ensure config is an object
    if (!config) {
      config = {};
    }

    // Ensure paths and apis properties exist and are arrays
    if (!config.paths) {
      config.paths = [];
    }
    if (!config.dbs) {
      config.dbs = [];
    }
    if (!config.apis) {
      config.apis = [];
    }

    // Update config - TODO: make sure that it's passing a mutable reference
    _writeInterface(config, logger, interfaceName, interfaceType, selectedType);

    const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
    fs.writeFileSync(configPath, updatedContent);
    logger.appendLine("[INFO] Config updated successfully");

    interfacesProvider.refresh();
  } catch (error) {
    logger.appendLine(`[INFO] Failed to update config: ${error}`);
  }
}

function getList(interfaceType: wasm.InterfaceType): string[] {
  const list =
    interfaceType === wasm.InterfaceType.Api
      ? apiList
      : interfaceType === wasm.InterfaceType.Database
      ? dbList
      : interfaceType === wasm.InterfaceType.Storage
      ? apiList // TODO: This will lead to an error once we introduce the storage
      : apiList;
  return list;
}

function getPlaceholderName(interfaceType: wasm.InterfaceType): string {
  const name =
    interfaceType === wasm.InterfaceType.Api
      ? "Api"
      : interfaceType === wasm.InterfaceType.Database
      ? "Datastore"
      : interfaceType === wasm.InterfaceType.Storage
      ? "Storage"
      : "api";
  return name;
}

function _writeInterface(
  config: any,
  logger: vscode.OutputChannel,
  interfaceName: string,
  interfaceType: wasm.InterfaceType,
  selectedType: string,
): void {
  const folderName =
    interfaceType === wasm.InterfaceType.Api
      ? "apis"
      : interfaceType === wasm.InterfaceType.Database
      ? "dbs"
      : interfaceType === wasm.InterfaceType.Storage
      ? "storages"
      : "apis";

  // Create schemas folder
  getOrCreateSchemasPath(interfaceName, folderName);

  // Modify paths
  config.paths.push({
    name: interfaceName,
    path: `.neat/${folderName}/${interfaceName}`,
  });

  if (interfaceType === wasm.InterfaceType.Api) {
    config.apis.push({
      name: interfaceName,
      apiType: wasm.apiTypeFromFriendlyUX(selectedType),
    });
  }

  if (interfaceType === wasm.InterfaceType.Database) {
    config.dbs.push({
      name: interfaceName,
      dbType: wasm.dbTypeFromFriendlyUX(selectedType),
    });
  }

  if (interfaceType === wasm.InterfaceType.Storage) {
    // TODO
  }
}
