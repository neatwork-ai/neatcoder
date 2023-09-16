import * as wasm from "./../pkg/neatcoder";
import { window, workspace } from "vscode";
import * as fs from "fs";
import * as path from "path";
import { getRoot, readAppState, saveAppStateToFile } from "./utils";
import { TaskPoolProvider } from "./providers/taskPool";
import { TasksCompletedProvider } from "./providers/tasksCompleted";
import { toTaskView } from "./models/task";
import { makeRequest, makeStreamingRequest } from "./httpClient";
import { scanSourceFolder, streamCode } from "./commands/streamCode";
import { logger } from "./logger";

/**
 * A class to manage the application state, including functionalities such as
 * handling tasks, schemas, interfaces, and initiating various jobs. This manager
 * is responsible for managing the state as well as calling methods on the
 * WASM AppState struct
 */
export class AppStateManager {
  private appState: wasm.AppState;
  private taskPoolProvider: TaskPoolProvider;
  private tasksCompletedProvider: TasksCompletedProvider;

  /**
   * Constructor to initialize the AppStateManager with given providers and a state read from a file.
   *
   * @param {TaskPoolProvider} taskPoolProvider - The task pool provider instance.
   * @param {TasksCompletedProvider} tasksCompletedProvider - The tasks completed provider instance.
   */
  constructor(
    taskPoolProvider: TaskPoolProvider,
    tasksCompletedProvider: TasksCompletedProvider
  ) {
    this.appState = readAppState();
    this.taskPoolProvider = taskPoolProvider;
    this.tasksCompletedProvider = tasksCompletedProvider;

    // Update providers
    this.refresh();
  }

  /**
   * Gets the read-only application state.
   *
   * @returns {wasm.AppState} - The application state.
   */
  public getReadOnlyState(): wasm.AppState {
    return this.appState;
  }

  /**
   * Adds a new language to the application state.
   *
   * @param {wasm.Language} language - The new language to be added.
   */
  public addLanguage(language: wasm.Language) {
    this.appState.setLanguage(language);
    saveAppStateToFile(this.appState);
  }

  /**
   * Adds a new schema associated with a specific interface in the application state.
   *
   * @param {string} interfaceName - The name of the interface where the schema should be added.
   * @param {string} schemaName - The name of the new schema.
   * @param {string} schema - The schema definition.
   */
  public addSchema(interfaceName: string, schemaName: string, schema: string) {
    this.appState.addSchema(interfaceName, schemaName, schema);
    saveAppStateToFile(this.appState);
  }

  /**
   * Removes a schema associated with a specific interface from the application state.
   *
   * @param {string} interfaceName - The name of the interface where the schema should be removed from.
   * @param {string} schemaName - The name of the schema to be removed.
   */
  public removeSchema(interfaceName: string, schemaName: string) {
    this.appState.removeSchema(interfaceName, schemaName);
    saveAppStateToFile(this.appState);
  }

  /**
   * Adds a new interface to the application state.
   *
   * @param {wasm.Interface} newInterface - The new interface to be added.
   */
  public addInterface(newInterface: wasm.Interface) {
    this.appState.addInterface(newInterface);
    saveAppStateToFile(this.appState);
  }

  /**
   * Removes an interface from the application state.
   *
   * @param {string} interfaceName - The name of the interface to be removed.
   */
  public removeInterface(interfaceName: string) {
    this.appState.removeInterface(interfaceName);
    saveAppStateToFile(this.appState);
  }

  /**
   * Removes a task with a specific ID from the task pool in the application state.
   *
   * @param {number} taskId - The ID of the task to be removed.
   */
  public removeTask(taskId: number) {
    this.appState.removeTodo(taskId);
    saveAppStateToFile(this.appState);

    // Update providers
    this.refresh();
  }

  /**
   * Removes all tasks from the task pool in the application state.
   */
  public removeAllTasks() {
    this.appState.removeAllTodos();
    saveAppStateToFile(this.appState);

    // Update providers
    this.refresh();
  }

  /**
   * Initiates a job based on the task ID and the associated task parameters.
   *
   * @param {number} taskId - The ID of the task to start.
   * @param {wasm.OpenAI} llmClient - The OpenAI client instance.
   * @param {wasm.OpenAIParams} llmParams - The parameters for the OpenAI client.
   * @returns {Promise<void>} - A promise indicating the completion of the job.
   */
  public async startJob(
    taskId: number,
    llmClient: wasm.OpenAI,
    llmParams: wasm.OpenAIParams
  ): Promise<void> {
    const task = this.appState.popTodo(taskId);
    const taskType = task.taskType();
    const taskParams = task.taskParams;
    this.appState.addDone(task);

    this.refresh();

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

      if (taskType === wasm.TaskType.CodeGen) {
        // If a new file should be created (or overwritten)
        const relPath: string = taskParams.streamCode!.filename;

        const filePath = path.join(getRoot(), "src", relPath);

        const directoryPath = path.dirname(filePath);

        if (!fs.existsSync(directoryPath)) {
          fs.mkdirSync(directoryPath, { recursive: true }); // recursive ensures that nested directories are created
        }

        const tokenWriter = fs.createWriteStream(filePath, { flags: "w" });

        // Open the file in the editor
        logger.appendLine(`[INFO] Opening code editor`);
        const activeTextDocument = await workspace.openTextDocument(filePath);

        logger.appendLine(`[INFO] Showing text`);
        await window.showTextDocument(activeTextDocument, {
          preview: false,
        });

        const codebase = await scanSourceFolder();

        logger.appendLine(`[INFO] Making StreamCode call to WASM Module.`);
        let requestBody = this.appState.streamCode(
          llmClient,
          llmParams,
          taskParams,
          codebase
        );

        await makeStreamingRequest(requestBody, activeTextDocument).catch(
          console.error
        );
      }

      saveAppStateToFile(this.appState);
      this.refresh();
    } catch (error) {
      console.error("Error while performing Task:", error);
      throw error;
    }

    // Update providers
    this.refresh();
  }

  /**
   * Starts a prompt with the given OpenAI client, parameters, and user input.
   *
   * @param {wasm.OpenAI} llmClient - The OpenAI client instance.
   * @param {wasm.OpenAIParams} llmParams - The parameters for the OpenAI client.
   * @param {string} userInput - The user input to start the prompt with.
   */
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

  /**
   * Initiates a scaffold project operation using specified OpenAI client, parameters, and user input.
   * This method creates necessary task parameters and invokes the scaffold project method from the appState object.
   *
   * @param {wasm.OpenAI} llmClient - The OpenAI client instance to be used in this operation.
   * @param {wasm.OpenAIParams} llmParams - The parameters for the OpenAI client.
   * @param {string} userInput - The user input string.
   */
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

  /**
   * Initiates a build execution plan operation using specified OpenAI client and parameters.
   * This method invokes the buildExecutionPlan method from the appState object.
   *
   * @param {wasm.OpenAI} llmClient - The OpenAI client instance to be used in this operation.
   * @param {wasm.OpenAIParams} llmParams - The parameters for the OpenAI client.
   */
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

  /**
   * Handles the update operation of the task pool.
   * It retrieves the list of todo tasks and updates the task pool provider with the new list.
   */
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

  /**
   * Event handler for updating the tasks completed view.
   * It retrieves the list of completed tasks and updates the tasks completed provider with the new list.
   */
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

  /**
   * Refreshes the task pool and completed tasks views.
   */
  private refresh(): void {
    this.handleUpdateTaskPool();
    this.handleUpdateTasksCompleted();
  }
}
