import * as vscode from "vscode";
import * as wasm from "./../../pkg/neatcoder";
import { AppStateManager } from "../appStateManager";
import { addLanguage } from "./addLanguage";
import { startLoading, stopLoading } from "../statusBar";

export async function startPrompt(
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appManager: AppStateManager
): Promise<void> {
  {
    try {
      await addLanguage(appManager);
    } catch (error) {
      vscode.window.showErrorMessage(`Error: ${error}`);
      throw error;
    }

    const userInput = await vscode.window.showInputBox({
      prompt: "Input your project description",
      value:
        "The API service will provide functionalities for managing an e-commerce platform...",
    });

    if (userInput !== undefined) {
      startLoading();
      await appManager.startPrompt(llmClient, llmParams, userInput);
      stopLoading();

      vscode.window.showInformationMessage(
        `The project scaffold & execution plan is now available.`
      );
    } else {
      vscode.window.showErrorMessage("Unable to parse prompt.");
    }
  }
}
