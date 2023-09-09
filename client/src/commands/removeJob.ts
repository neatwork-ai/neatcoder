import { TaskView } from "../models/task";
import { AppStateManager } from "../appStateManager";

export async function removeJob(
  taskView: TaskView,
  appManager: AppStateManager
): Promise<void> {
  const taskId = taskView.task.id;
  appManager.removeTask(taskId);
}
