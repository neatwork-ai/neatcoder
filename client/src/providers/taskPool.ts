import * as vscode from "vscode";
import { TaskView, toTaskView } from "../models/task";
import * as wasm from "../../pkg/neatcoder";

/// Implements a data provider for the view
export class TaskPoolProvider implements vscode.TreeDataProvider<TaskView> {
  tasks: TaskView[] = [];

  private _onDidChangeTreeData: vscode.EventEmitter<
    TaskView | undefined | null | void
  > = new vscode.EventEmitter<TaskView | undefined | null | void>();
  readonly onDidChangeTreeData: vscode.Event<
    TaskView | undefined | null | void
  > = this._onDidChangeTreeData.event;

  constructor() {}

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
