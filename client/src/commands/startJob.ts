import {
  OutputChannel,
  Position,
  TextDocument,
  window,
  workspace,
} from "vscode";
import * as fs from "fs";
import * as path from "path";
import { TaskView } from "../models/task";
import * as wasm from "../../pkg/neatcoder";
import { scanSourceFolder, streamCode } from "./streamCode";
import { makeRequest } from "../httpClient";

export async function startJob(
  taskView: TaskView,
  llmClient: wasm.OpenAI,
  llmParams: wasm.OpenAIParams,
  appState: wasm.AppState,
  logger: OutputChannel
): Promise<void> {
  const taskId = taskView.task.id;
  const taskType = taskView.task.taskParams.task_type; // TODO: change to camelCase

  logger.appendLine(`[INFO] Sending StartJob command for uuid ${taskId}`);

  appState.finishTaskById(taskId);

  if (taskType === wasm.TaskType.ScaffoldProject) {
    await appState.scaffoldProject(
      llmClient,
      llmParams,
      taskView.task.taskParams,
      makeRequest
    );
  }

  if (taskType === wasm.TaskType.BuildExecutionPlan) {
    await appState.buildExecutionPlan(llmClient, llmParams, makeRequest);
  }

  if (taskType === wasm.TaskType.CodeGen) {
    // If a new file should be created (or overwritten)
    const filePath: string = taskView.task.taskParams.streamCode!.filename;
    const tokenWriter = fs.createWriteStream(filePath, { flags: "w" });

    const directoryPath = path.dirname(filePath);
    if (!fs.existsSync(directoryPath)) {
      fs.mkdirSync(directoryPath, { recursive: true }); // recursive ensures that nested directories are created
    }

    // Open the file in the editor
    const activeTextDocument = await workspace.openTextDocument(filePath);

    await window.showTextDocument(activeTextDocument, {
      preview: false,
    });

    const codebase = await scanSourceFolder();

    await appState.streamCode(
      llmClient,
      llmParams,
      taskView.task.taskParams,
      codebase,
      (token: string) => {
        streamCode(token, activeTextDocument);
      }
    );
  }
}
