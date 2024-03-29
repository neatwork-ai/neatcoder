import * as wasm from "../../../pkg/neatcoder";
import { TaskView } from "../models";
import { appDataManager } from "../../core/appData";

/**
 * Asynchronously starts a job using specified parameters and manages it through the application state manager.
 *
 * @param taskView - The task view instance containing task details to be processed.
 * @param llmParams - The parameters that configure the OpenAI client for task processing.
 * @param appManager - The application state manager which is responsible for managing application state, including starting jobs.
 * @return Promise<void> - A promise that resolves once the job has been initiated.
 */
export async function runTask(
  taskView: TaskView,
  llmParams: wasm.OpenAIParams,
  appManager: appDataManager
): Promise<void> {
  const taskId = taskView.task!.id;

  await appManager.runTask(taskId, llmParams);
}
