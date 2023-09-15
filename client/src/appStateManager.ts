import * as wasm from "./../pkg/neatcoder";
import { OutputChannel, window, workspace, TextDocument } from "vscode";
import * as fs from "fs";
import * as path from "path";
import { getRoot, readAppState, saveAppStateToFile } from "./utils";
import { TaskPoolProvider } from "./providers/taskPool";
import { TasksCompletedProvider } from "./providers/tasksCompleted";
import { toTaskView } from "./models/task";
import { makeRequest, makeStreamingRequest } from "./httpClient";
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

  public addLanguage(language: wasm.Language) {
    this.appState.setLanguage(language);
    saveAppStateToFile(this.appState);
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

  public removeTask(taskId: number) {
    this.appState.removeTodo(taskId);
    saveAppStateToFile(this.appState);

    // Update providers
    this.refresh();
  }

  public removeAllTasks() {
    this.appState.removeAllTodos();
    saveAppStateToFile(this.appState);

    // Update providers
    this.refresh();
  }

  public async startJob(
    taskId: number,
    llmClient: wasm.OpenAI,
    llmParams: wasm.OpenAIParams
  ): Promise<void> {
    const task = this.appState.popTodo(taskId);
    const taskType = task.taskType();
    const taskParams = task.taskParams;

    this.refresh();

    this.logger.appendLine(`[DEBUG] Task TYPE ${taskType}`);
    this.logger.appendLine(`[DEBUG] Task Params ${taskParams}`);

    if (taskType === undefined) {
      window.showErrorMessage(`[ERROR] Task Type is undefined.`);
    }

    // The pattern matching should be offloaded to Rust
    try {
      if (taskType === wasm.TaskType.ScaffoldProject) {
        await this.appState.scaffoldProject(
          llmClient,
          llmParams,
          taskParams,
          makeRequest
        );
      }

      if (taskType === wasm.TaskType.BuildExecutionPlan) {
        await this.appState.buildExecutionPlan(
          llmClient,
          llmParams,
          makeRequest
        );
      }

      this.logger.appendLine(
        `Does ${taskType} == ${
          wasm.TaskType.CodeGen
        }? According to JS the answer is ${taskType === wasm.TaskType.CodeGen}`
      );

      if (taskType === wasm.TaskType.CodeGen) {
        this.logger.appendLine(`[DEBUG] We are preparing the codegen!`);

        // If a new file should be created (or overwritten)
        const relPath: string = taskParams.streamCode!.filename;
        this.logger.appendLine(`[DEBUG] relPath: ${relPath}`);

        const filePath = path.join(getRoot(), "src", relPath);

        const directoryPath = path.dirname(filePath);
        this.logger.appendLine(`[DEBUG] Directpath: ${directoryPath}`);
        this.logger.appendLine(
          `[DEBUG] Does it exist?: ${fs.existsSync(directoryPath)}`
        );

        if (!fs.existsSync(directoryPath)) {
          this.logger.appendLine(`[DEBUG] Creating path: ${filePath}`);
          fs.mkdirSync(directoryPath, { recursive: true }); // recursive ensures that nested directories are created
        }

        const tokenWriter = fs.createWriteStream(filePath, { flags: "w" });

        // Open the file in the editor
        this.logger.appendLine(`[INFO] Opening code editor`);
        const activeTextDocument = await workspace.openTextDocument(filePath);

        this.logger.appendLine(`[INFO] Showing text`);
        await window.showTextDocument(activeTextDocument, {
          preview: false,
        });

        this.logger.appendLine(`[INFO] Getting codebase...`);
        const codebase = await scanSourceFolder(this.logger);

        this.logger.appendLine(`[INFO] Making StreamCode call to wasm....`);
        let requestBody = this.appState.streamCode(
          llmClient,
          llmParams,
          taskParams,
          codebase
        );

        await makeStreamingRequest(
          requestBody,
          activeTextDocument,
          this.logger
        ).catch(console.error);
      }

      console.log("Adding Task to DONE");
      this.appState.addDone(task);
      console.log("Saving state");
      saveAppStateToFile(this.appState);
      console.log("Refreshing..");
      this.refresh();

      this.logger.appendLine(
        `[DEBUG] AppState ${JSON.stringify(this.appState.castToString())}`
      );
    } catch (error) {
      console.error("Error while performing Task:", error);
      throw error;
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

  private handleUpdateTaskPool(): void {
    try {
      const tasksTodo: wasm.Task[] = this.appState.getTodoTasks();

      // Update the local task list
      this.taskPoolProvider.tasks = toTaskView(tasksTodo);

      // Refresh the view
      this.taskPoolProvider.refresh();
    } catch (error) {
      console.error("Error while updating FE TaskPool:", error);
    }
  }

  // Event handler
  private handleUpdateTasksCompleted(): void {
    try {
      const tasksDone: wasm.Task[] = this.appState.getDoneTasks();

      window.showInformationMessage(
        `HERE ARE THE TASKS DONE: ${JSON.stringify(tasksDone)}`
      );

      // Update the local task list
      this.tasksCompletedProvider.tasks = toTaskView(tasksDone);

      // Refresh the view
      this.tasksCompletedProvider.refresh();
    } catch (error) {
      window.showInformationMessage(`HERRRRRRRO: ${error}`);

      console.error("Error while updating FE AuditPool:", error);
    }
  }

  private refresh(): void {
    this.handleUpdateTaskPool();
    this.handleUpdateTasksCompleted();
  }
}
