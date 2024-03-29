import * as vscode from "vscode";
import { TaskView } from "./models";

/**
 * Implements a data provider for displaying tasks in a tree view in a VS Code extension.
 * It manages a list of tasks and allows adding new tasks to the tree view.
 */
export class TaskPoolProvider implements vscode.TreeDataProvider<TaskView> {
  root: TaskView = new TaskView("root");

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

  getChildren(element?: TaskView): Thenable<TaskView[]> {
    if (!element || element === this.root) {
      return Promise.resolve(this.root.children);
    }
    return Promise.resolve(element.children);
  }
}

/**
 * This class implements a data provider to manage and display completed tasks
 * in a tree view within a VS Code extension. It handles a list of completed tasks
 * and allows adding new tasks to the list, which can then be displayed in the tree view.
 */
export class TasksCompletedProvider
  implements vscode.TreeDataProvider<TaskView>
{
  root: TaskView = new TaskView("root");

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

  getChildren(element?: TaskView): Thenable<TaskView[]> {
    if (!element || element === this.root) {
      return Promise.resolve(this.root.children);
    }
    return Promise.resolve(element.children);
  }
}
