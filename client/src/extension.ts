// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import * as fs from "fs";
import { TaskPoolProvider } from "./providers/taskPool";
import { InterfacesProvider } from "./providers/interfaces";
import { TasksCompletedProvider } from "./providers/tasksCompleted";
import { setupConfigWatcher } from "./watchers/configWatcher";
import { addSchema } from "./commands/schemas/addSchema";
import { setupSchemaWatchers } from "./watchers/schemaWatcher";
import { runTask } from "./commands/runTask";
import { initCodeBase } from "./commands/initCodeBase";
import { removeInterface } from "./commands/interfaces/removeInterface";
import { removeSchema } from "./commands/schemas/removeSchema";
import InterfaceItem from "./models/interfaceItem";
import { TaskView } from "./models/task";
import { addInterface } from "./commands/interfaces/addInterface";
import * as wasm from "./../pkg/neatcoder";
import { AppStateManager } from "./appStateManager";
import { getOrSetApiKey } from "./utils";
import { removeTask } from "./commands/removeTask";
import { removeAllTasks } from "./commands/removeAllTasks";
import { initStatusBar } from "./statusBar";
import { initLogger, logger } from "./logger";

let configWatcher: fs.FSWatcher | undefined;
const schemaWatchers: { [key: string]: fs.FSWatcher } = {};

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

  // Read or Initialize Application state

  let appManager = new AppStateManager(jobQueueProvider, auditTrailProvider);
  let llmClient = new wasm.OpenAI("TODO");
  let llmParams = wasm.OpenAIParams.empty(wasm.OpenAIModels.Gpt35Turbo16k);

  // === Setup File Watchers ===

  // Setup File Watcher which checks for changes in the `.neat` and
  // communicates them to the server if relevant
  setupSchemaWatchers(schemaWatchers, interfacesProvider, appManager);
  setupConfigWatcher(schemaWatchers, interfacesProvider, appManager);

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
    vscode.commands.registerCommand("extension.initCodeBase", async () => {
      initCodeBase(llmClient, llmParams, appManager);
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
        await runTask(taskView, llmClient, llmParams, appManager);
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

  Object.values(schemaWatchers).forEach((watcher) => watcher.close());
}
