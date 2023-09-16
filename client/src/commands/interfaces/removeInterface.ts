import * as vscode from "vscode";
import * as fs from "fs";
import * as rimraf from "rimraf"; // Used for recursive directory deletion
import InterfaceItem from "../../providers/interfaceItem";
import { getConfig, getOrCreateConfigPath, getPath } from "../../utils";
import { logger } from "../../logger";

export function removeInterface(item: InterfaceItem) {
  if (!item) {
    vscode.window.showErrorMessage("Please select an interface to remove.");
    return;
  }

  const interfaceName = item.label; // Extracting interface name from the clicked item
  const config = getConfig();

  // We need to get the variable before removing the interface from the config
  const schemasPathObj = config.paths.find(
    (entry: any) => entry.name === interfaceName
  );

  _removeInterface(config, interfaceName);

  // Delete the corresponding schemas folder.
  // Delete schemas comes after removing the interface as the
  // intention is that by removing the interface first the schema watchers
  // should be garbage collected
  _deleteSchemas(schemasPathObj, interfaceName);
}

function _deleteSchemas(schemasPathObj: any, interfaceName: string): void {
  if (!schemasPathObj) {
    // TODO: Incorrect because the folder still exists..
    logger.appendLine(
      `[WARN] No folder found for the interface ${interfaceName} schemas.`
    );
    vscode.window.showInformationMessage(
      `Interface named ${interfaceName} removed successfully.`
    );
    return;
  }

  const directoryToDelete = getPath(schemasPathObj.path);

  // Check if the directory exists before deleting
  if (fs.existsSync(directoryToDelete)) {
    rimraf.sync(directoryToDelete);
  } else {
    console.warn(`Directory ${directoryToDelete} does not exist.`);
  }

  vscode.window.showInformationMessage(
    `Interface named ${interfaceName} removed successfully.`
  );
}

function _removeInterface(config: any, interfaceName: string) {
  // Remove interface from dbs and apis
  config.dbs = config.dbs.filter((db: any) => db.name !== interfaceName);
  config.apis = config.apis.filter((api: any) => api.name !== interfaceName);

  // Remove interface from paths
  config.paths = config.paths.filter(
    (entry: any) => entry.name !== interfaceName
  );

  let configPath = getOrCreateConfigPath();

  // Write updated config back
  fs.writeFileSync(configPath, JSON.stringify(config, null, 4));
}
