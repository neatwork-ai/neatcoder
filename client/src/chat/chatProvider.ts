import * as vscode from "vscode";
import { getWebviewContent } from "./chat";

export class ChatGPTViewProvider implements vscode.WebviewViewProvider {
  private _view?: vscode.WebviewView;
  private _doc?: vscode.TextDocument;
  private readonly _extensionUri: vscode.Uri;

  constructor(extensionUri: vscode.Uri) {
    this._extensionUri = extensionUri;
  }

  public resolveWebviewView(
    webviewView: vscode.WebviewView,
    context: vscode.WebviewViewResolveContext,
    token: vscode.CancellationToken
  ) {
    this._view = webviewView;

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this._extensionUri],
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    // This should return your chat UI's HTML content.
    // You can utilize getWebviewContent function from the previous example.
    // If you have local resources, you need to adjust their paths using webview.asWebviewUri.
    return getWebviewContent();
  }
}
