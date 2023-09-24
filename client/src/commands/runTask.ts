import * as wasm from "../../pkg/neatcoder";
import { TaskView } from "../models/task";
import { AppStateManager } from "../appStateManager";

/**
 * Asynchronously starts a job using specified parameters and manages it through the application state manager.
 *
 * @param taskView - The task view instance containing task details to be processed.
 * @param llmClient - An instance of OpenAI client to facilitate the task processing.
 * @param llmParams - The parameters that configure the OpenAI client for task processing.
 * @param appManager - The application state manager which is responsible for managing application state, including starting jobs.
 * @return Promise<void> - A promise that resolves once the job has been initiated.
 */
export async function runTask(
  taskView: TaskView,
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appManager: AppStateManager
): Promise<void> {
  const taskId = taskView.task!.id;

  await appManager.runTask(taskId, llmClient, llmParams);
}
