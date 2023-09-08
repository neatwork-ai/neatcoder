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
import { startJob } from "./commands/startJob";
import { startPrompt } from "./commands/startPrompt";
import { setupSrcFolderWatcher } from "./watchers/sourceWatcher";
import { removeInterface } from "./commands/interfaces/removeInterface";
import { removeSchema } from "./commands/schemas/removeSchema";
import InterfaceItem from "./providers/interfaceItem";
import * as wasm from "./../pkg/neatcoder";
import { readAppState } from "./utils";
import { TaskView } from "./models/task";
import { addInterface } from "./commands/interfaces/addInterface";

import fetch from "node-fetch";
import { Headers, Request } from "node-fetch";

(global as any).fetch = fetch;
(global as any).Headers = Headers;
(global as any).Request = Request;

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

  // Read or Initialize Application state
  let appState = readAppState();
  let llmClient = new wasm.OpenAI("TODO");
  let llmParams = wasm.OpenAIParams.empty(wasm.OpenAIModels.Gpt35Turbo16k);

  // Create the output channel for logging
  let logger = vscode.window.createOutputChannel("Neatcoder");
  logger.appendLine("[INFO] Extension Name: Neatcoder");

  // === Init Providers ===

  const jobQueueProvider = new TaskPoolProvider(appState, logger);
  const auditTrailProvider = new TasksCompletedProvider(appState, logger);
  const interfacesProvider = new InterfacesProvider();

  // === Setup File Watchers ===

  // Setup File Watcher which checks for changes in the `.neat` and
  // communicates them to the server if relevant
  setupSchemaWatchers(schemaWatchers, interfacesProvider, appState, logger);
  setupConfigWatcher(schemaWatchers, interfacesProvider, appState, logger);
  setupSrcFolderWatcher(logger);

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
    vscode.window.registerTreeDataProvider("jobQueueView", jobQueueProvider)
  );

  context.subscriptions.push(
    vscode.window.registerTreeDataProvider("auditTrailView", auditTrailProvider)
  );

  context.subscriptions.push(
    vscode.window.registerTreeDataProvider("interfacesView", interfacesProvider)
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.startPrompt", () => {
      startPrompt(llmClient, llmParams, appState, logger);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addDatastore", () => {
      addInterface(wasm.InterfaceType.Database, interfacesProvider, logger);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addApi", () => {
      addInterface(wasm.InterfaceType.Api, interfacesProvider, logger);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addSchema", addSchema)
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeInterface",
      (item: InterfaceItem) => {
        removeInterface(item, logger);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.startJob",
      (taskView: TaskView) => {
        startJob(taskView, llmClient, llmParams, appState, logger);
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
    vscode.commands.registerCommand("extension.debugAppState", function () {
      const jsonString = appState.castToString();
      vscode.window.showInformationMessage(jsonString);
      logger.appendLine(jsonString);
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
