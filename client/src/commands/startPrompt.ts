import * as vscode from "vscode";
import * as wasm from "./../../pkg/neatcoder";
import { saveAppStateToFile } from "../utils";
import { makeRequest } from "../httpClient";

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
      await scaffold(llmClient, llmParams, appState, userInput);
      saveAppStateToFile(appState);

      // Use the TCP client to send the command
      logger.appendLine(`[INFO] Sending InitPrompt command via TCP Socket`);
    } else {
      vscode.window.showErrorMessage("Unable to parse prompt.");
    }
  }
}

async function scaffold(
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appState: wasm.AppState,
  userInput: string
) {
  const taskType = wasm.TaskType.ScaffoldProject;

  const taskPayload = new wasm.TaskParamsInner(
    new wasm.ScaffoldParams(userInput)
  );
  const taskParams = new wasm.TaskParams(taskType, taskPayload);

  try {
    await appState.scaffoldProject(
      llmClient,
      llmParams,
      taskParams,
      makeRequest
    );
  } catch (error) {
    console.error("Error occurred:", error);
  }
}

async function schedule(
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appState: wasm.AppState,
  userInput: string
) {
  const taskType = wasm.TaskType.BuildExecutionPlan;

  const taskPayload = new wasm.TaskParamsInner();
  const taskParams = new wasm.TaskParams(taskType, taskPayload);

  try {
    await appState.scaffoldProject(
      llmClient,
      llmParams,
      taskParams,
      makeRequest
    );
  } catch (error) {
    console.error("Error occurred:", error);
  }
}
