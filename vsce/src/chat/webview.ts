import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";
import * as wasm from "../../pkg/neatcoder";

export function setWebviewContent(
  panel: vscode.WebviewPanel,
  context: vscode.ExtensionContext,
  chatHistory?: wasm.Chat
) {
  const reactBuildPath = path.join(
    context.extensionPath,
    "..",
    "webview/build/"
  ); // Assuming 'webview' is where you copied your build files
  const entryHtml = path.join(reactBuildPath, "index.html");

  let content = fs.readFileSync(entryHtml, "utf8");

  // Use asWebviewUri to get the correct URI for the assets
  content = content.replace(/src="\/static\/js\/(.*?)"/g, (match, filename) => {
    const scriptSrc = panel.webview.asWebviewUri(
      vscode.Uri.file(path.join(reactBuildPath, "static", "js", filename))
    );
    return `src="${scriptSrc}"`;
  });

  content = content.replace(
    /href="\/static\/css\/(.*?)"/g,
    (match, filename) => {
      const cssSrc = panel.webview.asWebviewUri(
        vscode.Uri.file(path.join(reactBuildPath, "static", "css", filename))
      );
      return `href="${cssSrc}"`;
    }
  );

  const publicPath = vscode.Uri.file(reactBuildPath);
  const webviewPath = panel.webview.asWebviewUri(publicPath);

  // Injecting the public path
  const inlineScript = `<script>window.publicPath = "${webviewPath.toString()}";</script>`;
  content = content.replace(
    '<script id="pathInjection"></script>',
    inlineScript
  );

  if (chatHistory) {
    const historyScript = `<script>window.initialChatHistory = ${JSON.stringify(
      chatHistory.messages
    )};</script>`;
    // Insert the script before your React app's root div or the first script tag
    content = content.replace(
      '<div id="root"></div>',
      `<div id="root"></div>${historyScript}`
    );
  }

  panel.webview.html = content;
}
