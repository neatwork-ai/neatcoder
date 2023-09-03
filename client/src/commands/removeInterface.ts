import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import * as rimraf from "rimraf"; // Used for recursive directory deletion
import InterfaceItem from "../providers/interfaceItem";
import { getConfig, getOrCreateConfigPath, getPath } from "../utils";

export function removeInterface(
  item: InterfaceItem,
  logger: vscode.OutputChannel
) {
  if (!item) {
    vscode.window.showErrorMessage("Please select an interface to remove.");
    return;
  }

  const interfaceName = item.label; // Extracting interface name from the clicked item

  const config = getConfig();

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

  // Delete the corresponding schemas folder

  const schemasPathObj = config.paths.find(
    (entry: any) => entry.name === interfaceName
  );
  if (!schemasPathObj) {
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
