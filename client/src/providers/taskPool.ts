import * as vscode from "vscode";
import { TaskView } from "../models/task";

/**
 * Implements a data provider for displaying tasks in a tree view in a VS Code extension.
 * It manages a list of tasks and allows adding new tasks to the tree view.
 */
export class TaskPoolProvider implements vscode.TreeDataProvider<TaskView> {
  /** Array holding the tasks to be displayed in the tree view. */
  tasks: TaskView[] = [];

  /**
   * Private property holding an event emitter instance to notify about tree data changes.
   */
  private _onDidChangeTreeData: vscode.EventEmitter<
    TaskView | undefined | null | void
  > = new vscode.EventEmitter<TaskView | undefined | null | void>();

  /**
   * Public readonly property to expose the event for external listeners to detect tree data changes.
   */
  readonly onDidChangeTreeData: vscode.Event<
    TaskView | undefined | null | void
  > = this._onDidChangeTreeData.event;

  /**
   * Constructor to initialize a new instance of the TaskPoolProvider class.
   */
  constructor() {}

  /**
   * Method to refresh the tree view. It triggers a refresh of the tree data by firing an event.
   */
  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  /**
   * Method to retrieve a tree item given a TaskView element.
   * It simply returns the passed tree item.
   *
   * @param element - The TaskView item to retrieve.
   * @returns The retrieved tree item.
   */
  getTreeItem(element: TaskView): vscode.TreeItem {
    return element;
  }

  /**
   * Method to get the children of a tree item.
   * If no element is provided, it returns the root tasks.
   * Otherwise, it returns an empty array since tasks don't have child elements in this implementation.
   *
   * @param element - The parent tree item for which to retrieve children.
   * @returns A promise resolving to an array of child TaskView objects (or an empty array).
   */
  getChildren(element?: TaskView): Thenable<TaskView[]> {
    if (!element) {
      return Promise.resolve(this.tasks);
    }
    return Promise.resolve([]);
  }

  /**
   * Method to add a new task to the task pool and refresh the tree view to include the new task.
   *
   * @param task - The new TaskView object to be added to the task pool.
   */
  addTask(task: TaskView) {
    this.tasks.push(task);
    this.refresh();
  }
}
