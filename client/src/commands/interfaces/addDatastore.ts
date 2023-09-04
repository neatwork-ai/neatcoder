import * as vscode from "vscode";
import * as fs from "fs";
import {
  getOrCreateConfigPath,
  getOrCreateDatastoreSchemaPath,
} from "../../utils";
import { dbList } from "../../models/mappings";
import * as wasm from "../../../pkg/neatcoder";

/**
 * Presents a quick pick list to the user to select or input a datastore type.
 * Handles the user's selection/input and updates the appropriate configuration.
 *
 * @returns {void}
 */
export function addDatastore(
  appState: wasm.AppState,
  logger: vscode.OutputChannel
): void {
  {
    const quickPick = vscode.window.createQuickPick();
    quickPick.items = dbList.map((label) => ({ label }));
    quickPick.placeholder = "Select or type a datastore type";
    quickPick.matchOnDescription = true;
    quickPick.matchOnDetail = true;

    let customEntered = false;

    // If the user types in the QuickPick box and the value doesn't match an item, show a "(custom)" option
    quickPick.onDidChangeValue((value) => {
      if (dbList.includes(value)) {
        customEntered = false;
        quickPick.items = dbList.map((item) => ({ label: item }));
      } else {
        customEntered = true;
        quickPick.items = [
          ...dbList.map((item) => ({ label: item })),
          { label: `${value} (custom)` },
        ];
      }
    });

    // Runs once the user proceeds by click enter or left-click with the mouse
    quickPick.onDidAccept(() => {
      let selectedDbType: string;
      logger.appendLine("BOINK");

      if (
        quickPick.selectedItems[0] &&
        dbList.includes(quickPick.selectedItems[0].label)
      ) {
        logger.appendLine("BUNKER IF");
        customEntered = false;
        selectedDbType = quickPick.selectedItems[0].label;
      } else {
        logger.appendLine("BUNKER ELSE");
        selectedDbType = customEntered
          ? quickPick.value
          : quickPick.selectedItems[0].label;
      }

      // If the selected item has "(custom)", trim that part out
      if (selectedDbType.endsWith(" (custom)")) {
        logger.appendLine("BUNKER TONY");
        selectedDbType = selectedDbType.substring(
          0,
          selectedDbType.lastIndexOf(" (custom)")
        );
      }

      // Convert the string into DbType
      logger.appendLine(`Selected Datastore type ${selectedDbType}`);
      const dbType: wasm.DbType = wasm.dbTypeFromFriendlyUX(selectedDbType);
      handleDatastoreSelection(appState, dbType, logger);
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
async function handleDatastoreSelection(
  appState: wasm.AppState,
  dbType: wasm.DbType,
  logger: vscode.OutputChannel
): Promise<void> {
  try {
    const datastoreName = await vscode.window.showInputBox({
      prompt: "Enter the name of the datastore",
      placeHolder: "Datastore name",
    });
    if (datastoreName === undefined) {
      return; // User canceled the input box
    }

    if (!datastoreName) {
      vscode.window.showErrorMessage("Datastore name cannot be empty!");
      return;
    }

    // Update Runtime State
    const db = new wasm.Database(
      datastoreName,
      dbType,
      // undefined,
      // undefined,
      {}
    );
    const dbInterface = wasm.Interface.newDb(db);
    appState.addInterface(dbInterface);

    // Persist state
    const configPath = getOrCreateConfigPath();
    getOrCreateDatastoreSchemaPath(datastoreName);

    const content = await vscode.workspace.fs.readFile(
      vscode.Uri.file(configPath)
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
      config.apis = [];
    }

    // Modify paths
    logger.appendLine(`AAAA`);
    config.paths.push({
      name: datastoreName,
      path: `.neat/dbs/${datastoreName}`,
    });

    // Modify dbs (you can modify this based on additional inputs if required)
    logger.appendLine(`BBBB`);
    config.dbs.push({
      name: datastoreName,
      dbType: dbType, // This assumes that your selection from quick pick is the dbType
      // ... other properties like port and host can be added as needed
    });

    const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
    // await vscode.workspace.fs.writeFile(
    //   vscode.Uri.file(configPath),
    //   updatedContent
    // );
    fs.writeFileSync(configPath, updatedContent);
    logger.appendLine("Config updated successfully!");
  } catch (error) {
    logger.appendLine(`Failed to update config: ${error}`);
  }
}
