import * as vscode from "vscode";
import * as wasm from "./../../pkg";
import { saveAppStateToFile } from "../utils";

export async function startPrompt(
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appState: wasm.AppState,
  logger: vscode.OutputChannel
): Promise<void> {
  {
    const userInput = await vscode.window.showInputBox({
      prompt: "Input your project description",
      value:
        "The API service will provide functionalities for managing an e-commerce platform...",
    });

    if (userInput !== undefined) {
      const taskType = wasm.TaskType.ScaffoldProject;
      const taskPayload = new wasm.ScaffoldParams(userInput);
      const taskParams = new wasm.TaskParams(taskType, taskPayload); // TODO: THE ERROR IS HERE.

      await appState.scaffoldProject(llmClient, llmParams, taskParams);
      saveAppStateToFile(appState);

      // Use the TCP client to send the command
      logger.appendLine(`[INFO] Sending InitPrompt command via TCP Socket`);
    } else {
      vscode.window.showErrorMessage("Unable to parse prompt.");
    }
  }
}
