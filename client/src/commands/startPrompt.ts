import * as vscode from "vscode";
import * as wasm from "./../../pkg";

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
      const taskPayload = new wasm.ScaffoldProject(userInput);
      const taskParams = new wasm.TaskParams(taskType, taskPayload);

      await appState.scaffoldProject(llmClient, llmParams, taskParams);

      // Use the TCP client to send the command
      logger.appendLine(`[INFO] Sending InitPrompt command via TCP Socket`);
    } else {
      vscode.window.showErrorMessage("Unable to parse prompt.");
    }
  }
}
