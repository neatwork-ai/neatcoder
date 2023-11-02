// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";
import * as fs from "fs";
import * as wasm from "../pkg/neatcoder";
import {
  InterfacesProvider,
  addSchema,
  removeInterface,
  removeSchema,
  addInterface,
  InterfaceItem,
} from "./foreignInterfaces";
import {
  runTask,
  retryTask,
  removeTask,
  removeAllTasks,
  runAllTasks,
  TaskView,
  TaskPoolProvider,
  TasksCompletedProvider,
} from "./taskPool";
import { initCodeBase, appDataManager, setupDotNeatWatcher } from "./core";
import { getOrSetApiKey, initStatusBar, initLogger, logger } from "./utils";
import { ChatProvider, initChat, setupChatWatcher } from "./chat";
import { getOrSetModelVersion, setModelVersion } from "./utils/utils";
import MixpanelHelper from "./utils/mixpanelHelper";

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
  const chatProvider = new ChatProvider();

  // init Mixpanel
  let mixpanel = MixpanelHelper.getInstance();
  mixpanel.trackEvent("activate");

  // Read or Initialize Application state

  let appManager = new appDataManager(jobQueueProvider, auditTrailProvider);
  let llmParams = await getLLMParams();

  logger.appendLine("[INFO] Extension Activated. llmParams: " + llmParams);

  // === Setup File Watchers ===

  // Setup File Watcher which checks for changes in the `.neat` and
  // communicates them to the server if relevant
  setupDotNeatWatcher(
    schemaWatchers,
    interfacesProvider,
    chatProvider,
    appManager
  );
  chatWatcher = setupChatWatcher(chatProvider);

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
    vscode.window.registerTreeDataProvider("chatTreeView", chatProvider)
  );

  // Register the Chat command
  vscode.commands.registerCommand("extension.createChat", () => {
    mixpanel.trackEvent('createChat');
    initChat(context);
  });

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.initCodeBase", async () => {
      let llmParams = await getLLMParams();
      initCodeBase(llmParams, appManager);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addDatastore", async () => {
      mixpanel.trackEvent('addDatastore');
      addInterface(wasm.InterfaceType.Database, interfacesProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addApi", async () => {
      mixpanel.trackEvent('addApi');
      addInterface(wasm.InterfaceType.Api, interfacesProvider);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.addSchema", async (item: InterfaceItem) => {
      mixpanel.trackEvent('addSchema');
      await addSchema(item);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeInterface",
      (item: InterfaceItem) => {
        mixpanel.trackEvent('removeInterface');
        removeInterface(item);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeSchema",
      (item: InterfaceItem) => {
        mixpanel.trackEvent('removeSchema');
        removeSchema(item);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.runTask",
      async (taskView: TaskView) => {
        mixpanel.trackEvent('runTask');
        let llmParams = await getLLMParams();
        await runTask(taskView, llmParams, appManager);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.removeTask",
      (taskView: TaskView) => {
        mixpanel.trackEvent('removeTask');
        removeTask(taskView, appManager);
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.removeAllTasks", () => {
      mixpanel.trackEvent('removeAllTasks');
      removeAllTasks(appManager);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.chooseModel", async () => {
      mixpanel.trackEvent('chooseModel');
      await setModelVersion();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("extension.runAllTasks", async () => {
      mixpanel.trackEvent('runAllTasks');
      let llmParams = await getLLMParams();
      runAllTasks(llmParams, appManager);
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "extension.retryTask",
      async (taskView: TaskView) => {
        mixpanel.trackEvent('retryTask');
        let llmParams = await getLLMParams();
        await retryTask(taskView, llmParams, appManager);
      }
    )
  );
}

async function getLLMParams(): Promise<wasm.OpenAIParams> {
  let modelVersion = await getOrSetModelVersion();
  if (modelVersion === null) {
    modelVersion = wasm.OpenAIModels.Gpt4;
    vscode.window.showErrorMessage(
      "Invalid model version, defaulting to Gpt4."
    );
  }
  return wasm.OpenAIParams.empty(modelVersion);
}

// This method is called when the extension is deactivated
export function deactivate() {
  let mixpanel = MixpanelHelper.getInstance();
  mixpanel.trackEvent("deactivate");

  if (configWatcher) {
    configWatcher.close();
  }

  if (chatWatcher) {
    chatWatcher.dispose();
  }

  Object.values(schemaWatchers).forEach((watcher) => watcher.close());
}
