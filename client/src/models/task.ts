/// Defines data model for the JobQueue view
import * as vscode from "vscode";
import * as wasm from "../../pkg/neatcoder";

export class TaskView extends vscode.TreeItem {
  task?: wasm.Task;
  children: TaskView[] = []; // Add children for directory representation

  constructor(label: string, task?: wasm.Task) {
    super(
      label,
      task
        ? vscode.TreeItemCollapsibleState.None
        : vscode.TreeItemCollapsibleState.Collapsed
    );

    this.task = task;
    if (task) {
      this.iconPath = new vscode.ThemeIcon("zap");
      this.contextValue = "taskItem";
    } else {
      this.iconPath = new vscode.ThemeIcon("folder"); // You can choose an appropriate icon for folders
      this.contextValue = "groupItem";
    }
  }
}

// export function toTaskView(tasks: wasm.Task[]): TaskView[] {
//   return tasks.map((task) => new TaskView(task.name, task));
// }

// export function buildTreeFromTasks(tasks: wasm.Task[]): TaskView {
//   const root = new TaskView("root");

//   for (const task of tasks) {
//     let parts = task.name.split("/");
//     let currentNode = root;

//     for (let i = 0; i < parts.length; i++) {
//       const part = parts[i];
//       let childNode = currentNode.children.find(
//         (child) => child.label === part
//       );

//       if (!childNode) {
//         if (i === parts.length - 1) {
//           // It's a file
//           childNode = new TaskView(part, task);
//         } else {
//           // It's a directory
//           childNode = new TaskView(part);
//         }
//         currentNode.children.push(childNode);
//       }

//       currentNode = childNode;
//     }
//   }

//   return root;
// }

export function buildTreeFromTasks(tasks: wasm.Task[]): TaskView {
  const root = new TaskView("root");

  for (const task of tasks) {
    let parts = task.name.split("/");
    let currentNode = root;

    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      let childNode = currentNode.children.find(
        (child) => child.label === part
      );

      if (!childNode) {
        if (i === parts.length - 1) {
          // It's a file
          childNode = new TaskView(part, task);
        } else {
          // It's a directory
          childNode = new TaskView(part);
        }
        currentNode.children.push(childNode);
      }

      currentNode = childNode;
    }
  }

  // Sort the tree such that directories come before files
  function sortTree(node: TaskView) {
    node.children.sort((a, b) => {
      if (!a.task && b.task) return -1; // a is a directory, b is a file
      if (a.task && !b.task) return 1; // a is a file, b is a directory
      return (a.label as string).localeCompare(b.label as string); // Both are of the same type, sort by label
    });

    // Recursively sort the children
    for (const child of node.children) {
      sortTree(child);
    }
  }

  sortTree(root);

  return root;
}
