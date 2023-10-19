// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import * as fs from "fs";
import * as wasm from "../pkg/neatcoder";
import { InterfacesProvider } from "./foreignInterfaces/providers";
import { addSchema } from "./foreignInterfaces/commands/addSchema";
import { runTask } from "./taskPool/commands/runTask";
import { initCodeBase } from "./core/workflows/initCodeBase";
import { removeInterface } from "./foreignInterfaces/commands/removeInterface";
import { removeSchema } from "./foreignInterfaces/commands/removeSchema";
import InterfaceItem from "./foreignInterfaces/models";
import { TaskView } from "./taskPool/models";
import { addInterface } from "./foreignInterfaces/commands/addInterface";
import { appDataManager } from "./core/appData";
import { getOrSetApiKey, storeChat } from "./utils/utils";
import { removeTask } from "./taskPool/commands/removeTask";
import { removeAllTasks } from "./taskPool/commands/removeAllTasks";
import { initStatusBar } from "./utils/statusBar";
import { initLogger, logger } from "./utils/logger";
import { ChatTreeViewProvider } from "./chat/providers";
import { initChat } from "./chat/commands";
import { TaskPoolProvider, TasksCompletedProvider } from "./taskPool/providers";
import { setupDotNeatWatcher } from "./core/watchers";
import { setupChatWatcher } from "./chat/watchers";

export const activePanels: Map<number, vscode.WebviewPanel> = new Map();
export const chats = new wasm.Chats();

// Declare activePanels at the top-level to make it accessible throughout your extension's main script.
let configWatcher: fs.FSWatcher | undefined; // TODO: remove, not being used.
const schemaWatchers: { [key: string]: fs.FSWatcher } = {};
let chatWatcher: vscode.FileSystemWatcher | undefined;

// This method is called when your extension is activated
export async function activate(context: vscode.ExtensionContext) {
  if (!vscode.workspace.workspaceFolders) {
    vscode.window.showErrorMessage(
      "Please open a workspace folder before using this extension."
    );
    return;
  }

  initStatusBar(context);
  initLogger(context);
  getOrSetApiKey();

  // Create the output channel for logging
  logger.appendLine("[INFO] Extension Name: Neatcoder");

  // === Init Providers ===

  const jobQueueProvider = new TaskPoolProvider();
  const auditTrailProvider = new TasksCompletedProvider();
  const interfacesProvider = new InterfacesProvider();
  const chatTreeProvider = new ChatTreeViewProvider();

  // Read or Initialize Application state

  let appManager = new appDataManager(jobQueueProvider, auditTrailProvider);
  let llmParams = wasm.OpenAIParams.empty(wasm.OpenAIModels.Gpt35Turbo16k);

  // === Setup File Watchers ===

  // Setup File Watcher which checks for changes in the `.neat` and
  // communicates them to the server if relevant
  setupDotNeatWatcher(schemaWatchers, interfacesProvider, appManager);
  chatWatcher = setupChatWatcher(chatTreeProvider);

  // === Registration & Garbage Collection ===

  // The `register` methods return a disposable object. A disposable object is
  // something that has a dispose method on it, which should be called when the
  // resource is no longer needed to free up resources.
  //
  // We take the returned disposables and add them to the `context.subscriptions`.
  // In the context of a VS Code extension's activate function, the context
  // object is provided, and it has a subscriptions property which is an
  // array of disposables. When the extension is deactivated
  // everything in `context.subscriptions` will be disposed of automatically.
  context.subscriptions.push(
    vscode.window.registerTreeDataProvider("taskPoolView", jobQueueProvider)
  );

  context.subscriptions.push(
    vscode.window.registerTreeDataProvider("auditTrailView", auditTrailProvider)
  );

  context.subscriptions.push(
    vscode.window.registerTreeDataProvider("interfacesView", interfacesProvider)
  );

  context.subscriptions.push(
    vscode.window.registerTreeDataProvider("chatTreeView", chatTreeProvider)
  );

  // Register the Chat command
  vscode.commands.registerCommand("extension.createChat", () => {
    initChat(context);
  });

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.initCodeBase", async () => {
      initCodeBase(llmParams, appManager);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addDatastore", async () => {
      addInterface(wasm.InterfaceType.Database, interfacesProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addApi", async () => {
      addInterface(wasm.InterfaceType.Api, interfacesProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addSchema", addSchema)
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeInterface",
      (item: InterfaceItem) => {
        removeInterface(item);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeSchema",
      (item: InterfaceItem) => {
        removeSchema(item);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.runTask",
      async (taskView: TaskView) => {
        await runTask(taskView, llmParams, appManager);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeTask",
      (taskView: TaskView) => {
        removeTask(taskView, appManager);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.removeAllTasks", () => {
      removeAllTasks(appManager);
    })
  );
}

// This method is called when the extension is deactivated
export function deactivate() {
  if (configWatcher) {
    configWatcher.close();
  }

  if (chatWatcher) {
    chatWatcher.dispose();
  }

  Object.values(schemaWatchers).forEach((watcher) => watcher.close());
}
