// export { ChatTreeViewProvider } from "./providers";
export { initChat } from "./commands";
export { setupChatWatcher } from "./watchers";
export { ChatProvider } from "./providers";
import * as vscode from "vscode";

export const activePanels: Map<number, vscode.WebviewPanel> = new Map();
