import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import * as pako from "pako";
import * as wasm from "../../pkg/neatcoder";
import { ApiEntry, DbEntry, PathEntry } from "../foreignInterfaces/providers";

/// ===== Read ===== ///

export function readAppState(): wasm.AppState {
  const root = getRoot();
  const filePath = path.join(root, ".neat/cache", "state");

  // Check if the file exists
  if (!fs.existsSync(filePath)) {
    return wasm.AppState.empty();
  }

  try {
    // Read the file content
    const binaryData = fs.readFileSync(filePath);

    // Deserialize the data
    const appState = deserializeAppState(binaryData);

    return appState;
  } catch (e) {
    vscode.window.showErrorMessage(`Failed to Retrieve cached data: ${e}`);
    throw e;
  }
}

function readDirectoryStructure(
  dir: string,
  current: Record<string, string>
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

export function saveCump(appState: wasm.AppState): void {
  const payload = serializeAppState(appState);
  saveFile(payload, ".neat/cache", "state");
}

function saveFile(
  payload: ArrayBuffer,
  folder: string,
  filename: string
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
  try {
    const jsonString = appState.castToString();
    const compressedData = pako.gzip(jsonString);
    return compressedData.buffer as ArrayBuffer;
  } catch (e) {
    vscode.window.showErrorMessage(`Serialization failed:, ${e}`);
    throw e;
  }
}

function deserializeAppState(buffer: ArrayBuffer): wasm.AppState {
  try {
    const decompressedData = pako.ungzip(new Uint8Array(buffer));
    const jsonString = new TextDecoder().decode(decompressedData);
    const appState = wasm.AppState.castFromString(jsonString);
    return appState;
  } catch (e) {
    vscode.window.showErrorMessage(`Deserialization failed:, ${e}`);
    throw e;
  }
}

/// ===== Getters && Others ===== ///

export function generateSrcRecord(): Record<string, string> {
  const rootDir = "src";
  const structure: Record<string, string> = {};

  readDirectoryStructure(rootDir, structure);

  return structure;
}

export function getConfigIfAny(): {
  paths: PathEntry[];
  dbs: DbEntry[];
  apis: ApiEntry[];
} | null {
  const root = getRoot();

  const neatPath = path.join(root, ".neat");
  const configPath = path.join(neatPath, "config.json");

  let config;
  if (!fs.existsSync(configPath)) {
    return null;
  } else {
    const configContent = fs.readFileSync(configPath, "utf-8");
    config = JSON.parse(configContent);

    // Ensure config is an object
    if (!config) {
      return null;
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
      `Configuration file created at .neat/config.json`
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
  folderName: string
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

export function getOrSetApiKey(): any {
  let config = vscode.workspace.getConfiguration("extension");
  let apiKey = config.get("apiKey");

  if (!apiKey) {
    vscode.window
      .showInputBox({
        prompt: "Please enter your API key",
        ignoreFocusOut: true,
      })
      .then((value) => {
        if (value) {
          config.update("apiKey", value, vscode.ConfigurationTarget.Global);
          vscode.window.showInformationMessage("API key saved!");
        } else {
          // Handle the case where the input box was dismissed without entering a value
          vscode.window.showErrorMessage(
            "API key is required to use this extension."
          );
        }
      });
  }

  return apiKey;
}

export async function getChat(uri: vscode.Uri): Promise<wasm.Chat> {
  try {
    const data = await vscode.workspace.fs.readFile(uri);
    const content = Buffer.from(data).toString("utf8");
    return wasm.Chat.castFromString(content);
  } catch (error) {
    console.error("Failed to get chat:", error);
    throw new Error(`Failed to get chat from ${uri.path}: ${error}`);
  }
}

export async function storeChat(name: string, chat: wasm.Chat): Promise<void> {
  try {
    // Convert the chat instance to a JSON string
    const jsonString = chat.castToString();

    // Build the file path
    const folderPath = path.join(
      vscode.workspace.rootPath || "",
      ".neat",
      "chats"
    );
    const filePath = path.join(folderPath, `${name}.json`);

    // Ensure the directory exists
    await vscode.workspace.fs.createDirectory(vscode.Uri.file(folderPath));

    // Write the JSON string to the file
    const data = Buffer.from(jsonString, "utf8");
    await vscode.workspace.fs.writeFile(vscode.Uri.file(filePath), data);
  } catch (error) {
    console.error("Failed to store chat:", error);
    throw new Error(`Failed to store chat to ${name}.json: ${error}`);
  }
}
