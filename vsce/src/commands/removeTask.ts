import { TaskView } from "../models/task";
import { AppStateManager } from "../appStateManager";

/**
 * Asynchronously removes all tasks from the application state manager.
 *
 * @param appManager - The instance of AppStateManager from which all tasks will be removed.
 * @return Promise<void> - A promise that resolves once all tasks are removed.
 */
export async function removeTask(
  taskView: TaskView,
  appManager: AppStateManager
): Promise<void> {
  const taskId = taskView.task!.id;
  appManager.removeTask(taskId);
}
