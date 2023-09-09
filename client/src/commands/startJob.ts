import { OutputChannel } from "vscode";
import * as wasm from "../../pkg/neatcoder";
import { TaskView } from "../models/task";
import { AppStateManager } from "../appStateManager";

export async function startJob(
  taskView: TaskView,
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appManager: AppStateManager
): Promise<void> {
  const taskId = taskView.task.id;

  await appManager.startJob(taskId, llmClient, llmParams);
}
