export { ChatTreeViewProvider } from "./providers";
export { initChat } from "./commands";
export { setupChatWatcher } from "./watchers";
import * as vscode from "vscode";

export const activePanels: Map<number, vscode.WebviewPanel> = new Map();
