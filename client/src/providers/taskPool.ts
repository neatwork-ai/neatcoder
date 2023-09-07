import * as vscode from "vscode";
import { TaskView, toTaskView } from "../models/task";
import * as wasm from "../../pkg/neatcoder";

/// Implements a data provider for the view
export class TaskPoolProvider implements vscode.TreeDataProvider<TaskView> {
  private tasks: TaskView[] = [];

  private _onDidChangeTreeData: vscode.EventEmitter<
    TaskView | undefined | null | void
  > = new vscode.EventEmitter<TaskView | undefined | null | void>();
  readonly onDidChangeTreeData: vscode.Event<
    TaskView | undefined | null | void
  > = this._onDidChangeTreeData.event;

  logger: vscode.OutputChannel;

  constructor(
    private appState: wasm.AppState,
    logger: vscode.OutputChannel,
  ) {
    this.logger = logger;

    // Bind the event listeners
    this.appState.subscribe(() => {
      this.handleUpdateTaskPool(this.logger);
    });
  }

  // Event handler
  private handleUpdateTaskPool(logger: vscode.OutputChannel): void {
    const tasksTodo: wasm.Task[] = this.appState.getTodoTasks();

    // Update the local task list
    this.tasks = toTaskView(tasksTodo);

    // Maybe refresh the view if you're using this.tasks to display something in VSCode
    this.refresh();
  }

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: TaskView): vscode.TreeItem {
    return element;
  }

  getChildren(element?: TaskView): Thenable<TaskView[]> {
    if (!element) {
      return Promise.resolve(this.tasks);
    }
    return Promise.resolve([]);
  }

  addTask(task: TaskView) {
    this.tasks.push(task);
    this.refresh();
  }
}
