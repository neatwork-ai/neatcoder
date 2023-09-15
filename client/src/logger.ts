import * as vscode from "vscode";

export let logger: vscode.OutputChannel;

export function initLogger(context: vscode.ExtensionContext) {
  logger = vscode.window.createOutputChannel("Neatcoder");

  context.subscriptions.push(logger);
}
