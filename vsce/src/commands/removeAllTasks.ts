import { AppStateManager } from "../appStateManager";

export async function removeAllTasks(
  appManager: AppStateManager
): Promise<void> {
  appManager.removeAllTasks();
}
