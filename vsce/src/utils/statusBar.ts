import * as vscode from "vscode";

/**
 * An instance of the StatusBarItem class, representing a status bar item which can be added to the VS Code status bar.
 * @type {vscode.StatusBarItem}
 */
export let statusBar: vscode.StatusBarItem;

/**
 * A NodeJS timer that will be used to handle the loading animation interval.
 * @type {NodeJS.Timeout}
 */
let loadingInterval: NodeJS.Timeout;

/**
 * Initializes a status bar item with alignment to the right and a priority of 100, then adds it to the extension context's subscriptions.
 * This ensures that the status bar item will be disposed of when the extension is deactivated.
 *
 * @param {vscode.ExtensionContext} context - The context of the VS Code extension.
 */
export function initStatusBar(context: vscode.ExtensionContext) {
  statusBar = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );

  context.subscriptions.push(statusBar);
}

/**
 * Stops the loading animation and sets the status bar item's text to "Status: OK" before hiding it.
 */
export function stopLoading() {
  if (statusBar) {
    clearInterval(loadingInterval);
    statusBar.text = "Status: OK";
    statusBar.hide();
  }
}

/**
 * Initiates a loading animation on the status bar item. The animation cycles through different steps,
 * changing the status bar item's text and icon at a regular interval of 500 milliseconds.
 */
export function startLoading(message: string) {
  if (statusBar) {
    clearInterval(loadingInterval);
  }

  if (statusBar) {
    statusBar.color = new vscode.ThemeColor("statusBarItem.warningForeground");
    statusBar.backgroundColor = new vscode.ThemeColor(
      "statusBarItem.warningBackground"
    );

    let loadingStep = 0;
    loadingInterval = setInterval(() => {
      statusBar.text = `${getLoadingStep(loadingStep)} ${message}...`;
      loadingStep = (loadingStep + 1) % 4; // cycle between 0 and 3
    }, 500); // update every 500 ms
    statusBar.show();
  }
}

/**
 * Returns a string representing a loading icon based on the current step of the loading animation.
 *
 * @param {number} step - The current step of the loading animation, cycling between 0 and 3.
 * @returns {string} - A string representing a VS Code icon.
 */
function getLoadingStep(step: number): string {
  switch (step) {
    case 0:
      return "$(circle-outline)";
    case 1:
      return "$(circle-filled)";
    case 2:
      return "$(circle-outline)";
    case 3:
      return "$(circle-small)";
    default:
      return "$(circle-outline)"; // added a default case to avoid undefined return
  }
}
