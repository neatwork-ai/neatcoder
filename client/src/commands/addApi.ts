import * as vscode from "vscode";
import * as wasm from "../../pkg/neatcoder";
import { apiList } from "../models/mappings";
import { getOrCreateApiSchemaPath, getOrCreateConfigPath } from "../utils";

/**
 * Presents a quick pick list to the user to select or input a datastore type.
 * Handles the user's selection/input and updates the appropriate configuration.
 *
 * @returns {void}
 */
export function addApi(appState: wasm.AppState): void {
  {
    // The items to show in the quick pick list
    const quickPick = vscode.window.createQuickPick();
    quickPick.items = apiList.map((label) => ({ label }));
    quickPick.placeholder = "Select or type a API type";
    quickPick.matchOnDescription = true;
    quickPick.matchOnDetail = true;

    let customEntered = false;

    // If the user types in the QuickPick box and the value doesn't match an item, show a "(custom)" option
    quickPick.onDidChangeValue((value) => {
      if (apiList.includes(value)) {
        customEntered = false;
        quickPick.items = apiList.map((item) => ({ label: item }));
      } else {
        customEntered = true;
        quickPick.items = [
          ...apiList.map((item) => ({ label: item })),
          { label: `${value} (custom)` },
        ];
      }
    });

    quickPick.onDidAccept(() => {
      let selectedApiType: string;

      if (
        quickPick.selectedItems[0] &&
        apiList.includes(quickPick.selectedItems[0].label)
      ) {
        customEntered = false;
        selectedApiType = quickPick.selectedItems[0].label;
      } else {
        selectedApiType = customEntered
          ? quickPick.value
          : quickPick.selectedItems[0].label;
      }

      // If the selected item has "(custom)", trim that part out
      if (selectedApiType.endsWith(" (custom)")) {
        selectedApiType = selectedApiType.substring(
          0,
          selectedApiType.lastIndexOf(" (custom)")
        );
      }

      // Convert the string into apiType
      const apiType = wasm.apiTypeFromFriendlyUX(selectedApiType);

      if (apiType) {
        handleApiSelection(appState, apiType);
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
 * @param {ApiType} apiType - The type of API selected by the user.
 * @returns {void}
 */
function handleApiSelection(
  appState: wasm.AppState,
  apiType: wasm.ApiType
): void {
  // Prompt for the datastore name
  vscode.window
    .showInputBox({
      prompt: "Enter the name of the API",
      placeHolder: "API name",
    })
    .then((apiName) => {
      if (apiName === undefined) {
        return; // User canceled the input box
      }

      if (!apiName) {
        vscode.window.showErrorMessage("API name cannot be empty!");
        return;
      }

      // Update Runtime State
      const api = new wasm.Api(apiName, apiType, undefined, undefined, {});
      const apiInterface = wasm.Interface.newApi(api);
      appState.addInterface(apiInterface);

      // Persist state
      const configPath = getOrCreateConfigPath();
      getOrCreateApiSchemaPath(apiName);

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
          if (!config.apis) {
            config.apis = [];
          }

          // Modify paths
          config.paths.push({
            name: apiName,
            path: `.neat/apis/${apiName}`,
          });

          // Modify apis (you can modify this based on additional inputs if required)
          config.apis.push({
            name: apiName,
            apiType: apiType, // This assumes that your selection from quick pick is the apiType
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
