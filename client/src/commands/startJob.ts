import { OutputChannel } from "vscode";
import * as wasm from "../../pkg/neatcoder";
import { TaskView } from "../models/task";
import { AppStateManager } from "../appStateManager";

export async function startJob(
  taskView: TaskView,
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appManager: AppStateManager,
  logger: OutputChannel
): Promise<void> {
  const taskId = taskView.task.id;

  await appManager.startJob(
    taskId,
    taskView.task.taskParams,
    llmClient,
    llmParams
  );
}
