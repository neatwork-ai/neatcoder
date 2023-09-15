import { AppStateManager } from "../appStateManager";

export async function removeAllJobs(
  appManager: AppStateManager
): Promise<void> {
  appManager.removeAllTasks();
}
