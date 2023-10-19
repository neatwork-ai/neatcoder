export { ChatTreeViewProvider } from "./providers";
export { initChat } from "./commands";
export { setupChatWatcher } from "./watchers";

import * as wasm from "../../pkg/neatcoder";
import * as vscode from "vscode";

export const activePanels: Map<number, vscode.WebviewPanel> = new Map();
export const chats = new wasm.Chats();
