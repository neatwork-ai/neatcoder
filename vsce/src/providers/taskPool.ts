import * as vscode from "vscode";
import { TaskView } from "../models/task";

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
