import { appDataManager } from "../../core/appData";

export async function removeAllTasks(
  appManager: appDataManager
): Promise<void> {
  appManager.removeAllTasks();
}
