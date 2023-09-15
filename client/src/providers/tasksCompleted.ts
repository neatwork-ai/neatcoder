import * as vscode from "vscode";
import { TaskView } from "../models/task";

/**
 * This class implements a data provider to manage and display completed tasks
 * in a tree view within a VS Code extension. It handles a list of completed tasks
 * and allows adding new tasks to the list, which can then be displayed in the tree view.
 */
export class TasksCompletedProvider
  implements vscode.TreeDataProvider<TaskView>
{
  /** Holds the list of completed tasks that will be displayed in the tree view. */
  tasks: TaskView[] = [];

  /**
   * A private event emitter instance which is used to notify when there are changes
   * to the tree data, allowing the tree view to refresh and display the updated data.
   */
  private _onDidChangeTreeData: vscode.EventEmitter<
    TaskView | undefined | null | void
  > = new vscode.EventEmitter<TaskView | undefined | null | void>();

  /**
   * A public readonly event that external listeners can subscribe to in order to
   * be notified of changes to the tree data.
   */
  readonly onDidChangeTreeData: vscode.Event<
    TaskView | undefined | null | void
  > = this._onDidChangeTreeData.event;

  /**
   * Constructor to initialize a new instance of the TasksCompletedProvider class.
   */
  constructor() {}

  /**
   * Triggers a refresh of the tree view, notifying listeners of the onDidChangeTreeData
   * event that there has been a change to the tree data.
   */
  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  /**
   * Returns a tree item instance for a given TaskView element. This tree item is then
   * used to represent the task in the tree view.
   *
   * @param element - The TaskView item to convert to a tree item.
   * @returns The tree item representing the given TaskView item.
   */
  getTreeItem(element: TaskView): vscode.TreeItem {
    return element;
  }

  /**
   * Returns the child tasks of a given tree item. If no tree item is specified,
   * it returns the list of all root tasks. Since tasks do not have child items
   * in this implementation, it returns an empty array if a task is specified.
   *
   * @param element - The parent tree item, if any.
   * @returns A promise that resolves to the array of child tasks.
   */
  getChildren(element?: TaskView): Thenable<TaskView[]> {
    if (!element) {
      return Promise.resolve(this.tasks);
    }
    return Promise.resolve([]);
  }

  /**
   * Adds a new task to the list of completed tasks and triggers a refresh of
   * the tree view so the new task is displayed.
   *
   * @param task - The new TaskView instance to add to the list of completed tasks.
   */
  addTask(task: TaskView) {
    this.tasks.push(task);
    this.refresh();
  }
}
