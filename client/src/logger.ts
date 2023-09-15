import * as vscode from "vscode";

/**
 * An instance of the OutputChannel class, which represents an output channel in VS Code,
 * where the extension can write messages to.
 * @type {vscode.OutputChannel}
 */
export let logger: vscode.OutputChannel;

/**
 * This function initializes the logger instance as a new output channel in VS Code with the name "Neatcoder".
 * It also adds the logger to the context's subscriptions, ensuring that the output channel is disposed of when the extension is deactivated.
 *
 * @param {vscode.ExtensionContext} context - The context of the VS Code extension.
 */
export function initLogger(context: vscode.ExtensionContext) {
  logger = vscode.window.createOutputChannel("Neatcoder");

  context.subscriptions.push(logger);
}
