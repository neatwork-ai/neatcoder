import { TaskView } from "../models";
import { appDataManager } from "../../core/appData";

/**
 * Asynchronously removes all tasks from the application state manager.
 *
 * @param appManager - The instance of appDataManager from which all tasks will be removed.
 * @return Promise<void> - A promise that resolves once all tasks are removed.
 */
export async function removeTask(
  taskView: TaskView,
  appManager: appDataManager
): Promise<void> {
  const taskId = taskView.task!.id;
  appManager.removeTask(taskId);
}
