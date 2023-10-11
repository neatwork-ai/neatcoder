import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";

export function setWebviewContent(
  panel: vscode.WebviewPanel,
  context: vscode.ExtensionContext
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

  const inlineScript = `<script>window.publicPath = "${webviewPath.toString()}";</script>`;
  content = content.replace(
    '<script id="pathInjection"></script>',
    inlineScript
  );

  panel.webview.html = content;
}
