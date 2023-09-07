import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import * as pako from "pako";
import * as wasm from "../pkg/neatcoder";

/// ===== Read ===== ///

export function readAppState(): wasm.AppState {
  const root = getRoot();
  const filePath = path.join(root, ".neat/cache", "state");

  // Check if the file exists
  if (!fs.existsSync(filePath)) {
    return wasm.AppState.empty();
  }

  // Read the file content
  const binaryData = fs.readFileSync(filePath);

  // Deserialize the data
  const stateData = deserializeAppState(binaryData);

  // Create a new AppState instance and populate it with the deserialized data

  // const todoPipeline = new wasm.Pipeline(
  //   stateData.taskPool.todo.task,
  //   stateData.taskPool.todo.order
  // );
  // const donePipeline = new wasm.Pipeline(
  //   stateData.done.task,
  //   stateData.done.order
  // );

  // const realTaskPool = new wasm.TaskPool(
  //   stateData.taskPool.counter,
  //   todoPipeline,
  //   donePipeline
  // );

  // const realTaskPool = new wasm.TaskPool(stateData.taskPool);

  vscode.window.showInformationMessage(
    `APP STATE!!: ${JSON.stringify(stateData)}`,
  );

  const stringifiedData = JSON.stringify(stateData, null, 2);
  try {
    const appState = new wasm.AppState(stringifiedData);
    return appState;
  } catch (e) {
    vscode.window.showInformationMessage(`ERROR: ${e}`);
    throw e;
  }

  // stateData.specs,
  // stateData.scaffold,
  // stateData.interfaces,

  //
  //
  // return appState;
  //TODO remove
}

function readDirectoryStructure(
  dir: string,
  current: Record<string, string>,
): void {
  const files = fs.readdirSync(dir);

  for (const file of files) {
    const filePath = path.join(dir, file);
    const isDirectory = fs.statSync(filePath).isDirectory();

    if (isDirectory) {
      readDirectoryStructure(filePath, current);
    } else {
      current[filePath] = fs.readFileSync(filePath, "utf-8");
    }
  }
}

/// ===== Write ===== ///

export function saveAppStateToFile(appState: wasm.AppState): void {
  const payload = serializeAppState(appState);
  saveFile(payload, ".neat/cache", "state");
}

function saveFile(
  payload: ArrayBuffer,
  folder: string,
  filename: string,
): void {
  const root = getRoot();

  const folderPath = path.join(root, folder);

  // Create the directory if it doesn't exist
  if (!fs.existsSync(folderPath)) {
    fs.mkdirSync(folderPath, { recursive: true });
  }

  const filePath = path.join(folderPath, filename);
  fs.writeFileSync(filePath, Buffer.from(payload));
}

/// ===== Serialize / Deserialize ===== ///

export function serializeAppState(appState: wasm.AppState): ArrayBuffer {
  const data = {
    specs: appState.specs,
    scaffold: appState.scaffold,
    interfaces: appState.interfaces,
    taskPool: appState.taskPool,
  };

  const jsonString = JSON.stringify(data, null, 2);
  const compressedData = pako.gzip(jsonString);
  return compressedData.buffer as ArrayBuffer;
}

export function deserializeAppState(buffer: ArrayBuffer): any {
  try {
    const decompressedData = pako.ungzip(new Uint8Array(buffer));
    const jsonString = new TextDecoder().decode(decompressedData);
    return JSON.parse(jsonString);
  } catch (e) {
    vscode.window.showErrorMessage(`Decompression failed:, ${e}`);
    throw e;
  }
}

function serializeString(data: string): ArrayBuffer {
  const jsonString = JSON.stringify(data);
  const compressedData = pako.deflate(jsonString);
  return compressedData.buffer as ArrayBuffer;
}

function deserializeString(buffer: ArrayBuffer): string {
  const decompressedData = pako.inflate(new Uint8Array(buffer));
  const jsonString = new TextDecoder().decode(decompressedData);
  return JSON.parse(jsonString) as string;
}

/// ===== Getters && Others ===== ///

export function generateSrcRecord(): Record<string, string> {
  const rootDir = "src";
  const structure: Record<string, string> = {};

  readDirectoryStructure(rootDir, structure);

  return structure;
}

export function getConfig(): any {
  const root = getRoot();

  const neatPath = path.join(root, ".neat");
  const configPath = path.join(neatPath, "config.json");

  let config;
  if (!fs.existsSync(configPath)) {
    // If the .neat directory doesn't exist, create it
    if (!fs.existsSync(neatPath)) {
      fs.mkdirSync(neatPath);
    }

    // Create default configuration
    config = {
      paths: [],
      apis: [],
    };

    // Write the default config to the file
    fs.writeFileSync(configPath, JSON.stringify(config, null, 4));
    vscode.window.showInformationMessage(
      `Configuration file created at ${configPath}`,
    );
  } else {
    const configContent = fs.readFileSync(configPath, "utf-8");
    config = JSON.parse(configContent);

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
  }
  return config;
}

export function getOrCreateConfigPath(): string {
  const root = getRoot();
  const neatPath = path.join(root, ".neat");
  const configPath = path.join(neatPath, "config.json");

  // Create the directory if it doesn't exist
  if (!fs.existsSync(neatPath)) {
    fs.mkdirSync(neatPath, { recursive: true });
  }

  // Create the directory if it doesn't exist
  if (!fs.existsSync(configPath)) {
    // Create default configuration
    const newConfig = {
      paths: [],
      dbs: [],
      apis: [],
    };

    // Write the default config to the file
    fs.writeFileSync(configPath, JSON.stringify(newConfig, null, 4));
    vscode.window.showInformationMessage(
      `Configuration file created at ${configPath}`,
    );
  }

  return configPath;
}

export function getPath(relPath: string): any {
  const root = getRoot();
  const fullPath = path.join(root, relPath);
  return fullPath;
}

export function getOrCreateApiSchemaPath(apiName: string): string {
  const root = getRoot();
  const apiPath = path.join(root, ".neat/apis", apiName);

  // Create the directory if it doesn't exist
  if (!fs.existsSync(apiPath)) {
    fs.mkdirSync(apiPath, { recursive: true });
  }
  return apiPath;
}

export function getOrCreateSchemasPath(
  interfaceName: string,
  folderName: string,
): string {
  const root = getRoot();
  const schemasPath = path.join(root, `.neat/${folderName}`, interfaceName);

  // Create the directory if it doesn't exist
  if (!fs.existsSync(schemasPath)) {
    fs.mkdirSync(schemasPath, { recursive: true });
  }
  return schemasPath;
}

export function getRoot(): string {
  if (!vscode.workspace.workspaceFolders) {
    vscode.window.showErrorMessage("No workspace found.");
    throw new Error("No workspace found.");
  }

  const root = vscode.workspace.workspaceFolders[0].uri.fsPath;
  return root;
}

export function getFilename(filepath: string): string {
  const parts = filepath.split(/[/\\]/);
  return parts[parts.length - 1];
}
