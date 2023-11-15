import * as wasm from "../../../pkg/neatcoder";
import { appDataManager } from "../../core/appData";

/**
 * Asynchronously run all jobs using specified parameters and manages it through the application state manager.
 *
 * @param llmParams - The parameters that configure the OpenAI client for task processing.
 * @param appManager - The application state manager which is responsible for managing application state, including starting jobs.
 * @return Promise<void> - A promise that resolves once the job has been initiated.
 */
export async function runAllTasks(
  llmParams: wasm.ChatParamsWasm,
  appManager: appDataManager
): Promise<void> {
  appManager.runAllTasks(llmParams);
}
