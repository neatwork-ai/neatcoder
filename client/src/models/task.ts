/// Defines data model for the JobQueue view
import * as vscode from "vscode";
import * as wasm from "../../pkg/neatcoder";

export class TaskView extends vscode.TreeItem {
  task: wasm.Task;

  constructor(label: string, task: wasm.Task) {
    super(label, vscode.TreeItemCollapsibleState.None);

    this.task = task;
    this.iconPath = new vscode.ThemeIcon("zap"); // 'zap' is the Codicon for thunder/lightning
  }
}

export function toTaskView(tasks: wasm.Task[]): TaskView[] {
  return tasks.map((task) => new TaskView(task.name, task));
}
