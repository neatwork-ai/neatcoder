import * as vscode from "vscode";
import * as wasm from "../../../pkg/neatcoder";
import { appDataManager } from "../appData";
import { addLanguage } from "../commands/addLanguage";
import { startLoading, stopLoading } from "../../utils/statusBar";

/**
 * Asynchronously initiates a prompt to gather user input and starts processing based on the input.
 * It goes through the following steps:
 * 1. Adds a language through the `addLanguage` function.
 * 2. Prompts the user to input their project description.
 * 3. If user input is received, it starts a loading indicator, triggers the start of a prompt process through the appDataManager, and stops the loading indicator once completed.
 *
 * @param llmParams - Parameters configuring the OpenAI client for task processing.
 * @param appManager - The appDataManager instance responsible for managing the application state, including starting prompts.
 * @returns Promise<void> - A promise that resolves once the prompt process has initiated and the user has been notified.
 */
export async function initCodeBase(
  llmParams: wasm.OpenAIParams,
  appManager: appDataManager
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
      startLoading("Prompting the LLM..");
      await appManager.initCodeBase(llmParams, userInput);
      stopLoading();

      vscode.window.showInformationMessage(
        `The project tasks are now available.`
      );
    }
  }
}
