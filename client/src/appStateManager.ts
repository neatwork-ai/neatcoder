import * as wasm from "./../pkg/neatcoder";
import {
  OutputChannel,
  Position,
  TextDocument,
  window,
  workspace,
} from "vscode";
import * as fs from "fs";
import * as path from "path";
import { readAppState, saveAppStateToFile } from "./utils";
import { TaskPoolProvider } from "./providers/taskPool";
import { TasksCompletedProvider } from "./providers/tasksCompleted";
import { toTaskView } from "./models/task";
import { makeRequest } from "./httpClient";
import { scanSourceFolder, streamCode } from "./commands/streamCode";

export class AppStateManager {
  private appState: wasm.AppState;
  private logger: OutputChannel;
  private taskPoolProvider: TaskPoolProvider;
  private tasksCompletedProvider: TasksCompletedProvider;

  constructor(
    logger: OutputChannel,
    taskPoolProvider: TaskPoolProvider,
    tasksCompletedProvider: TasksCompletedProvider
  ) {
    this.appState = readAppState();
    this.logger = logger;
    this.taskPoolProvider = taskPoolProvider;
    this.tasksCompletedProvider = tasksCompletedProvider;

    // Update providers
    this.refresh();
  }

  public getReadOnlyState(): wasm.AppState {
    return this.appState;
  }

  public addSchema(interfaceName: string, schemaName: string, schema: string) {
    this.appState.addSchema(interfaceName, schemaName, schema);
    saveAppStateToFile(this.appState);
  }

  public removeSchema(interfaceName: string, schemaName: string) {
    this.appState.removeSchema(interfaceName, schemaName);
    saveAppStateToFile(this.appState);
  }

  public addInterface(newInterface: wasm.Interface) {
    this.appState.addInterface(newInterface);
    saveAppStateToFile(this.appState);
  }

  public removeInterface(interfaceName: string) {
    this.appState.removeInterface(interfaceName);
    saveAppStateToFile(this.appState);
  }

  public async startJob(
    taskId: number,
    taskParams: wasm.TaskParams,
    llmClient: wasm.OpenAI,
    llmParams: wasm.OpenAIParams
  ): Promise<void> {
    this.logger.appendLine(
      `[INFO] Sending StartJob command for uuid ${taskId}`
    );

    const taskType = taskParams.task_type; // TODO: change to camelCase and typed
    this.appState.finishTaskById(taskId);

    if (taskType === wasm.TaskType.ScaffoldProject) {
      await this.appState.scaffoldProject(
        llmClient,
        llmParams,
        taskParams,
        makeRequest
      );
    }

    if (taskType === wasm.TaskType.BuildExecutionPlan) {
      await this.appState.buildExecutionPlan(llmClient, llmParams, makeRequest);
    }

    if (taskType === wasm.TaskType.CodeGen) {
      // If a new file should be created (or overwritten)
      const filePath: string = taskParams.streamCode!.filename;
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

      await this.appState.streamCode(
        llmClient,
        llmParams,
        taskParams,
        codebase,
        (token: string) => {
          streamCode(token, activeTextDocument);
        }
      );
    }

    // Update providers
    this.refresh();
  }

  public async startPrompt(
    llmClient: wasm.OpenAI,
    llmParams: wasm.OpenAIParams,
    userInput: string
  ) {
    await this.scaffoldProject(llmClient, llmParams, userInput);
    await this.buildExecutionPlan(llmClient, llmParams);
    saveAppStateToFile(this.appState);

    // Update providers
    this.refresh();
  }

  private handleUpdateTaskPool(): void {
    try {
      const tasksTodo: wasm.Task[] = this.appState.getTodoTasks();

      window.showInformationMessage(`[DEBUG] Here are the TODOs ${tasksTodo}`);

      // Update the local task list
      this.taskPoolProvider.tasks = toTaskView(tasksTodo);

      // Refresh the view
      this.taskPoolProvider.refresh();
    } catch (error) {
      console.error("Error while updating FE TaskPool:", error);
    }
  }

  async scaffoldProject(
    llmClient: wasm.OpenAI,
    llmParams: wasm.OpenAIParams,
    userInput: string
  ) {
    const taskType = wasm.TaskType.ScaffoldProject;

    const taskPayload = new wasm.TaskParamsInner(
      new wasm.ScaffoldParams(userInput)
    );
    const taskParams = new wasm.TaskParams(taskType, taskPayload);

    try {
      await this.appState.scaffoldProject(
        llmClient,
        llmParams,
        taskParams,
        makeRequest
      );
    } catch (error) {
      console.error("Error occurred:", error);
    }
  }

  async buildExecutionPlan(
    llmClient: wasm.OpenAI,
    llmParams: wasm.OpenAIParams
  ) {
    try {
      await this.appState.buildExecutionPlan(llmClient, llmParams, makeRequest);
    } catch (error) {
      console.error("Error occurred:", error);
    }
  }

  // Event handler
  private handleUpdateTasksCompleted(): void {
    const tasksDone: wasm.Task[] = this.appState.getDoneTasks();

    // Update the local task list
    this.tasksCompletedProvider.tasks = toTaskView(tasksDone);

    // Refresh the view
    this.tasksCompletedProvider.refresh();
  }

  private refresh(): void {
    this.handleUpdateTaskPool();
    this.handleUpdateTasksCompleted();
  }
}
