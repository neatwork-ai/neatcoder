import * as vscode from "vscode";
import {
  getOrCreateConfigPath,
  getOrCreateDatastoreSchemaPath,
} from "../utils";
import { dbList } from "../models/mappings";
import * as wasm from "../../pkg/neatcoder";

/**
 * Presents a quick pick list to the user to select or input a datastore type.
 * Handles the user's selection/input and updates the appropriate configuration.
 *
 * @returns {void}
 */
export function addDatastore(): void {
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

    quickPick.onDidAccept(() => {
      let selectedDbType: string;

      if (
        quickPick.selectedItems[0] &&
        dbList.includes(quickPick.selectedItems[0].label)
      ) {
        customEntered = false;
        selectedDbType = quickPick.selectedItems[0].label;
      } else {
        selectedDbType = customEntered
          ? quickPick.value
          : quickPick.selectedItems[0].label;
      }

      // If the selected item has "(custom)", trim that part out
      if (selectedDbType.endsWith(" (custom)")) {
        selectedDbType = selectedDbType.substring(
          0,
          selectedDbType.lastIndexOf(" (custom)")
        );
      }

      // Convert the string into DbType
      const dbType = wasm.dbTypeFromFriendlyUX(selectedDbType);

      if (dbType) {
        handleDatastoreSelection(dbType);
        quickPick.dispose();
      }
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
function handleDatastoreSelection(dbType: wasm.DbType) {
  // Prompt for the datastore name
  vscode.window
    .showInputBox({
      prompt: "Enter the name of the datastore",
      placeHolder: "Datastore name",
    })
    .then((datastoreName) => {
      if (datastoreName === undefined) {
        return; // User canceled the input box
      }

      if (!datastoreName) {
        vscode.window.showErrorMessage("Datastore name cannot be empty!");
        return;
      }

      const configPath = getOrCreateConfigPath();
      getOrCreateDatastoreSchemaPath(datastoreName);

      // Read the config file
      vscode.workspace.fs.readFile(vscode.Uri.file(configPath)).then(
        (content) => {
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
          config.paths.push({
            name: datastoreName,
            path: `.neat/dbs/${datastoreName}`,
          });

          // Modify dbs (you can modify this based on additional inputs if required)
          config.dbs.push({
            name: datastoreName,
            dbType: dbType, // This assumes that your selection from quick pick is the dbType
            // ... other properties like port and host can be added as needed
          });

          // Write back the modified config
          const updatedContent = Buffer.from(JSON.stringify(config, null, 4)); // 4 spaces indentation
          vscode.workspace.fs
            .writeFile(vscode.Uri.file(configPath), updatedContent)
            .then(
              () => {
                vscode.window.showInformationMessage(
                  "Config updated successfully!"
                );
              },
              (error) => {
                vscode.window.showErrorMessage(
                  `Failed to update config: ${error.message}`
                );
              }
            );
        },
        (error) => {
          vscode.window.showErrorMessage(
            `Failed to read config: ${error.message}`
          );
        }
      );
    });
}
