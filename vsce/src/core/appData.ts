import * as wasm from "../../pkg/neatcoder";
import { window, workspace } from "vscode";
import * as fs from "fs";
import * as path from "path";
import { getRoot, readappData, saveappDataToFile } from "../utils/utils";
import { buildTreeFromTasks } from "../taskPool/models";
import { makeRequest, makeStreamingRequest } from "../utils/httpClient";
import { scanSourceFolder, streamCode } from "./workflows/streamCode";
import { logger } from "../utils/logger";
import { startLoading } from "../utils/statusBar";
import {
  TaskPoolProvider,
  TasksCompletedProvider,
} from "../taskPool/providers";

/**
 * A class to manage the application state, including functionalities such as
 * handling tasks, schemas, interfaces, and initiating various tasks. This manager
 * is responsible for managing the state as well as calling methods on the
 * WASM AppData struct
 */
export class appDataManager {
  private appData: wasm.AppData;
  private taskPoolProvider: TaskPoolProvider;
  private tasksCompletedProvider: TasksCompletedProvider;

  /**
   * Constructor to initialize the appDataManager with given providers and a state read from a file.
   *
   * @param {TaskPoolProvider} taskPoolProvider - The task pool provider instance.
   * @param {TasksCompletedProvider} tasksCompletedProvider - The tasks completed provider instance.
   */
  constructor(
    taskPoolProvider: TaskPoolProvider,
    tasksCompletedProvider: TasksCompletedProvider
  ) {
    this.appData = readappData();
    this.taskPoolProvider = taskPoolProvider;
    this.tasksCompletedProvider = tasksCompletedProvider;

    // Update providers
    this.refresh();
  }

  /**
   * Gets the read-only application state.
   *
   * @returns {wasm.AppData} - The application state.
   */
  public getReadOnlyState(): wasm.AppData {
    return this.appData;
  }

  /**
   * Adds a new language to the application state.
   *
   * @param {wasm.Language} language - The new language to be added.
   */
  public addLanguage(language: wasm.Language) {
    this.appData.setLanguage(language);
    saveappDataToFile(this.appData);
  }

  /**
   * Adds a new schema associated with a specific interface in the application state.
   *
   * @param {string} interfaceName - The name of the interface where the schema should be added.
   * @param {string} schemaName - The name of the new schema.
   * @param {string} schema - The schema definition.
   */
  public addSchema(interfaceName: string, schemaName: string, schema: string) {
    this.appData.addSchema(interfaceName, schemaName, schema);
    saveappDataToFile(this.appData);
  }

  /**
   * Removes a schema associated with a specific interface from the application state.
   *
   * @param {string} interfaceName - The name of the interface where the schema should be removed from.
   * @param {string} schemaName - The name of the schema to be removed.
   */
  public removeSchema(interfaceName: string, schemaName: string) {
    this.appData.removeSchema(interfaceName, schemaName);
    saveappDataToFile(this.appData);
  }

  /**
   * Adds a new interface to the application state.
   *
   * @param {wasm.Interface} newInterface - The new interface to be added.
   */
  public addInterface(newInterface: wasm.Interface) {
    this.appData.addInterface(newInterface);
    saveappDataToFile(this.appData);
  }

  /**
   * Removes an interface from the application state.
   *
   * @param {string} interfaceName - The name of the interface to be removed.
   */
  public removeInterface(interfaceName: string) {
    this.appData.removeInterface(interfaceName);
    saveappDataToFile(this.appData);
  }

  /**
   * Removes a task with a specific ID from the task pool in the application state.
   *
   * @param {number} taskId - The ID of the task to be removed.
   */
  public removeTask(taskId: number) {
    this.appData.popTodo(taskId);
    saveappDataToFile(this.appData);

    // Update providers
    this.refresh();
  }

  /**
   * Removes all tasks from the task pool in the application state.
   */
  public removeAllTasks() {
    this.appData.removeAllTodos();
    saveappDataToFile(this.appData);

    // Update providers
    this.refresh();
  }

  /**
   * Runs all tasks in the task pool in the application state sequentially.
   *
   * @param {wasm.ChatParamsWasm} llmParams - The parameters for the OpenAI client.
   * @returns {Promise<void>} - A promise indicating the completion of all tasks.
   */
  public async runAllTasks(llmParams: wasm.ChatParamsWasm): Promise<void> {
    try {
      // Retrieve all tasks from the task pool.
      const tasks: wasm.Task[] = this.appData.getTodoTasks();

      if (tasks.length === 0) {
        window.showInformationMessage("No tasks to run in the task pool.");
        return;
      }

      // Run each task sequentially.
      for (const task of tasks) {
        const taskId = task.id; // Assuming task object has an id property
        await this.runTask(taskId, llmParams);
      }

      window.showInformationMessage("All tasks completed.");
    } catch (err) {
      console.error("Error while running all tasks:", err);
      window.showErrorMessage(`Error while running all tasks: ${err}`);
      throw new Error((err as Error).message);
    }
  }

  /**
   * Initiates a task based on the task ID and the associated task parameters.
   *
   * @param {number} taskId - The ID of the task to start.
   * @param {wasm.ChatParamsWasm} llmParams - The parameters for the OpenAI client.
   * @returns {Promise<void>} - A promise indicating the completion of the task.
   */
  public async runTask(
    taskId: number,
    llmParams: wasm.ChatParamsWasm
  ): Promise<void> {
    const task = this.appData.popTodo(taskId);
    const taskType = task.taskType();
    const taskParams = task.taskParams;

    this.refresh();
    try {
      if (taskType === undefined) {
        window.showErrorMessage(`[ERROR] Task Type is undefined.`);
        throw new Error(`[ERROR] Task Type is undefined.`);
      }

      // The pattern matching should be offloaded to Rust
      if (taskType === wasm.TaskType.ScaffoldProject) {
        await this.appData.scaffoldProject(llmParams, taskParams, makeRequest);
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

        await window.showTextDocument(activeTextDocument);
        const codebase = await scanSourceFolder();

        logger.appendLine(`[INFO] Making StreamCode call to WASM Module.`);
        startLoading("Awaiting Code Stream");

        let requestBody = this.appData.streamCode(
          llmParams,
          taskParams,
          codebase
        );

        await makeStreamingRequest(requestBody, activeTextDocument);
      }

      // Update Task Pool
      this.appData.addDone(task);
      // Persist updated state
      saveappDataToFile(this.appData);
    } catch (err) {
      this.appData.addBackTodo(task);
      console.error(`Error while performing task ${task.name}. ERROR: ${err}`);
      window.showErrorMessage(`Error while performing task: ${err}`);
      throw new Error((err as Error).message);
    }

    this.refresh(); // need to refresh to reflect the state rollback
  }

  /**
   * Retries a task based on the task ID and the associated task parameters.
   *
   * @param {number} taskId - The ID of the task to retry.
   * @param {wasm.ChatParamsWasm} llmParams - The parameters for the OpenAI client.
   * @returns {Promise<void>} - A promise indicating the completion of the task.
   */
  public async retryTask(
    taskId: number,
    llmParams: wasm.ChatParamsWasm
  ): Promise<void> {
    const task = this.appData.popDone(taskId);
    this.appData.addBackTodo(task);
    return this.runTask(taskId, llmParams);
  }

  /**
   * Starts a prompt with the given OpenAI client, parameters, and user input.
   *
   * @param {wasm.ChatParamsWasm} llmParams - The parameters for the OpenAI client.
   * @param {string} userInput - The user input to start the prompt with.
   */
  public async initCodeBase(llmParams: wasm.ChatParamsWasm, userInput: string) {
    try {
      await this.scaffoldProject(llmParams, userInput);
      saveappDataToFile(this.appData);

      // Update providers
      this.refresh();
    } catch (err) {
      console.error("Error while initialising codebase:", err);
      window.showErrorMessage(`Error while initialising codebase: ${err}`);
      throw new Error((err as Error).message);
    }
  }

  /**
   * Initiates a scaffold project operation using specified OpenAI client, parameters, and user input.
   * This method creates necessary task parameters and invokes the scaffold project method from the appData object.
   *
   * @param {wasm.ChatParamsWasm} llmParams - The parameters for the OpenAI client.
   * @param {string} userInput - The user input string.
   */
  async scaffoldProject(llmParams: wasm.ChatParamsWasm, userInput: string) {
    const taskType = wasm.TaskType.ScaffoldProject;

    const taskPayload = new wasm.TaskParamsInner(
      new wasm.ScaffoldParams(userInput)
    );
    const taskParams = new wasm.TaskParams(taskType, taskPayload);

    try {
      await this.appData.scaffoldProject(llmParams, taskParams, makeRequest);
    } catch (err) {
      console.error(`Error while scaffolding the project: ${err}`);
      throw new Error((err as Error).message);
    }
  }

  /**
   * Handles the update operation of the task pool.
   * It retrieves the list of todo tasks and updates the task pool provider with the new list.
   */
  private handleUpdateTaskPool(): void {
    try {
      const tasksTodo: wasm.Task[] = this.appData.getTodoTasks();

      // Update the local task list
      this.taskPoolProvider.root = buildTreeFromTasks(tasksTodo);

      // Refresh the view
      this.taskPoolProvider.refresh();
    } catch (err) {
      console.error(`Error while updating FE TaskPool: ${err}`);
      throw new Error((err as Error).message);
    }
  }

  /**
   * Event handler for updating the tasks completed view.
   * It retrieves the list of completed tasks and updates the tasks completed provider with the new list.
   */
  private handleUpdateTasksCompleted(): void {
    try {
      const tasksDone: wasm.Task[] = this.appData.getDoneTasks();

      // Update the local task list
      this.tasksCompletedProvider.root = buildTreeFromTasks(tasksDone);

      // Refresh the view
      this.tasksCompletedProvider.refresh();
    } catch (err) {
      console.error(`Error while updating FE AuditPool: ${err}`);
      throw new Error((err as Error).message);
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
