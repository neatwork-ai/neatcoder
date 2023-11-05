import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import * as pako from "pako";
import * as wasm from "../../pkg/neatcoder";
import { ApiEntry, DbEntry, PathEntry } from "../foreignInterfaces/providers";
import { ChatEntry } from "../chat/providers";

/// ===== Read ===== ///

export function readappData(): wasm.AppData {
  const root = getRoot();
  const filePath = path.join(root, ".neat/cache", "state");

  // Check if the file exists
  if (!fs.existsSync(filePath)) {
    return wasm.AppData.empty();
  }

  try {
    // Read the file content
    const binaryData = fs.readFileSync(filePath);

    // Deserialize the data
    const appData = deserializeappData(binaryData);

    return appData;
  } catch (err) {
    vscode.window.showErrorMessage(`Failed to Retrieve cached data: ${err}`);
    throw new Error((err as Error).message);
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

export function saveappDataToFile(appData: wasm.AppData): void {
  const payload = serializeappData(appData);
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

export function serializeappData(appData: wasm.AppData): ArrayBuffer {
  try {
    const jsonString = appData.castToString();
    const compressedData = pako.gzip(jsonString);
    return compressedData.buffer as ArrayBuffer;
  } catch (err) {
    vscode.window.showErrorMessage(`Serialization failed:, ${err}`);
    throw new Error((err as Error).message);
  }
}

function deserializeappData(buffer: ArrayBuffer): wasm.AppData {
  try {
    const decompressedData = pako.ungzip(new Uint8Array(buffer));
    const jsonString = new TextDecoder().decode(decompressedData);
    const appData = wasm.AppData.castFromString(jsonString);
    return appData;
  } catch (err) {
    vscode.window.showErrorMessage(`Deserialization failed:, ${err}`);
    throw new Error((err as Error).message);
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
  chats: ChatEntry[];
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

export function getOrInitConfig(): {
  paths: PathEntry[];
  dbs: DbEntry[];
  apis: ApiEntry[];
  chats: ChatEntry[];
} {
  getOrCreateConfigPath();

  return getConfigIfAny()!;
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

export function getOrSetApiKey(): Promise<any> {
  return new Promise((resolve, reject) => {
    let config = vscode.workspace.getConfiguration("extension");
    let apiKey = config.get("apiKey");

    if (apiKey) {
      // If the API key is already set, resolve the Promise immediately.
      resolve(apiKey);
    } else {
      // Show the input box to the user to enter the API key.
      vscode.window
        .showInputBox({
          prompt: "Please enter your API key",
          ignoreFocusOut: true,
        })
        .then((value) => {
          if (value) {
            // Update the configuration with the new API key.
            config
              .update("apiKey", value, vscode.ConfigurationTarget.Global)
              .then(
                () => {
                  vscode.window.showInformationMessage("API key saved!");
                  resolve(value); // Resolve the Promise with the new API key.
                },
                (error) => {
                  vscode.window.showErrorMessage(
                    `Failed to save API key: ${error}`
                  );
                  reject(error); // Reject the Promise if there was an error saving the API key.
                }
              );
          } else {
            // Handle the case where the input box was dismissed without entering a value.
            vscode.window.showErrorMessage(
              "API key is required to use this extension."
            );
            reject("API key not provided."); // Reject the Promise as no API key was provided.
          }
        });
    }
  });
}

export async function getOrSetModelVersion(): Promise<wasm.OpenAIModels | null> {
  let config = vscode.workspace.getConfiguration("extension");
  let modelVersion = config.get("modelVersion") as string;

  if (!modelVersion) {
    const value = await vscode.window.showQuickPick(
      ["gpt-3.5-turbo-16k", "gpt-4"],
      {
        canPickMany: false,
        placeHolder: "Select an OpenAI model", // This is the placeholder text
      }
    );
    if (value) {
      await config.update(
        "modelVersion",
        value,
        vscode.ConfigurationTarget.Global
      );
      vscode.window.showInformationMessage("Model Version saved!");
      return fromModelVersionToEnum(value);
    } else {
      // Handle the case where the input box was dismissed without entering a value
      vscode.window.showErrorMessage(
        "Model version is required to use this extension."
      );
      return null;
    }
  }

  return fromModelVersionToEnum(modelVersion);
}

export async function setModelVersion() {
  let config = vscode.workspace.getConfiguration("extension");

  const value = await vscode.window.showQuickPick(
    ["gpt-3.5-turbo-16k", "gpt-4"],
    {
      canPickMany: false,
      placeHolder: "Select an OpenAI model", // This is the placeholder text
    }
  );
  if (value) {
    await config.update(
      "modelVersion",
      value,
      vscode.ConfigurationTarget.Global
    );
    vscode.window.showInformationMessage("Model Version saved!");
  }
}

export function fromModelVersionToEnum(
  modelStr: string
): wasm.OpenAIModels | null {
  switch (modelStr) {
    case "gpt-4-32k":
      return wasm.OpenAIModels.Gpt432k;
    case "gpt-4":
      return wasm.OpenAIModels.Gpt4;
    case "gpt-3.5-turbo":
      return wasm.OpenAIModels.Gpt35Turbo;
    case "gpt-3.5-turbo-16k":
      return wasm.OpenAIModels.Gpt35Turbo16k;
    default:
      return null;
  }
}

export async function getChat(uri: vscode.Uri): Promise<wasm.Chat> {
  try {
    const data = await vscode.workspace.fs.readFile(uri);
    const content = Buffer.from(data).toString("utf8");
    return wasm.Chat.castFromString(content);
  } catch (err) {
    console.error("Failed to get chat:", err);
    throw new Error(`Failed to get chat from ${uri.path}: ${err}`);
  }
}

// The wrapper function that constructs the Uri from the chat ID
export async function getChatById(chatId: string): Promise<wasm.Chat> {
  const root = getRoot();
  const chatsDir = path.join(root, ".neat", "chats"); // The directory where chats are stored
  const fileName = `${chatId}.json`; // Assuming the ID should be the filename
  const filePath = path.join(chatsDir, fileName); // Construct the full file path

  // Use vscode.Uri.file to convert file path to a vscode.Uri
  const uri = vscode.Uri.file(filePath);

  // Use getChat function with the constructed Uri
  return getChat(uri);
}

export async function storeChat(chat: wasm.Chat): Promise<void> {
  try {
    const chatId = chat.sessionId;
    // Convert the chat instance to a JSON string
    const jsonString = chat.castToString();

    // Build the file path
    const folderPath = path.join(
      vscode.workspace.rootPath || "",
      ".neat",
      "chats"
    );
    const filePath = path.join(folderPath, `${chatId}.json`);

    // Ensure the directory exists
    await vscode.workspace.fs.createDirectory(vscode.Uri.file(folderPath));

    // Write the JSON string to the file
    const data = Buffer.from(jsonString, "utf8");
    await vscode.workspace.fs.writeFile(vscode.Uri.file(filePath), data);
  } catch (err) {
    console.error("Failed to store chat:", err);
    throw new Error(`Failed to store chat to ${name}.json: ${err}`);
  }
}
