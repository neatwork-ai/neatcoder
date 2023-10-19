import { AppStateManager } from "../../core/appData";

export async function removeAllTasks(
  appManager: AppStateManager
): Promise<void> {
  appManager.removeAllTasks();
}
