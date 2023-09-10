import * as vscode from "vscode";

export let statusBar: vscode.StatusBarItem;
let loadingInterval: NodeJS.Timeout;

export function initStatusBar(context: vscode.ExtensionContext) {
  statusBar = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100
  );

  context.subscriptions.push(statusBar);
}

export function stopLoading() {
  if (statusBar) {
    clearInterval(loadingInterval);
    statusBar.text = "Status: OK";
    statusBar.hide();
  }
}

export function startLoading() {
  console.log("We are in the loading scope..");

  if (statusBar) {
    statusBar.color = new vscode.ThemeColor("statusBarItem.warningForeground");
    statusBar.backgroundColor = new vscode.ThemeColor(
      "statusBarItem.warningBackground"
    );

    let loadingStep = 0;
    loadingInterval = setInterval(() => {
      statusBar.text = `${getLoadingStep(loadingStep)} Prompting LLM...`;
      loadingStep = (loadingStep + 1) % 4; // cycle between 0 and 3
    }, 500); // update every 500 ms
    statusBar.show();
  }
}
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
