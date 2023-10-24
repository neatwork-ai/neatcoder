import * as vscode from "vscode";
import { appDataManager } from "../appData";
import { logger } from "../../utils/logger";
import { langList } from "../../utils/consts";
import * as wasm from "../../../pkg/neatcoder";

/**
 * Prompts the user to select or enter a programming language through a QuickPick interface.
 * The function returns a promise that resolves once a language has been selected or entered,
 * and rejects if the QuickPick interface is closed without a selection.
 *
 * @param appManager - The application state manager instance where the selected or entered language will be added.
 * @return Promise<void> - A promise that resolves when a language is selected or entered, and rejects if the selection interface is closed without making a selection.
 */
export function addLanguage(appManager: appDataManager): Promise<void> {
  {
    return new Promise((resolve, reject) => {
      const list = langList;
      const placeholderName = "Programming Language";

      const quickPick = vscode.window.createQuickPick();
      quickPick.items = list.map((label) => ({ label }));

      quickPick.placeholder = `Select or type a ${placeholderName} type;`;
      quickPick.matchOnDescription = true;
      quickPick.matchOnDetail = true;

      let customEntered = false;
      let itemSelected = false;

      // If the user types in the QuickPick box and the value doesn't match an item, show a "(custom)" option
      quickPick.onDidChangeValue((value) => {
        if (list.includes(value)) {
          customEntered = false;
          quickPick.items = list.map((item) => ({ label: item }));
        } else {
          customEntered = true;
          quickPick.items = [
            ...list.map((item) => ({ label: item })),
            { label: `${value} (custom)` },
          ];
        }
      });

      // Runs once the user proceeds by click enter or left-click with the mouse
      quickPick.onDidAccept(() => {
        let selectedType: string;
        itemSelected = true;

        if (
          quickPick.selectedItems[0] &&
          list.includes(quickPick.selectedItems[0].label)
        ) {
          customEntered = false;
          selectedType = quickPick.selectedItems[0].label;
        } else {
          selectedType = customEntered
            ? quickPick.value
            : quickPick.selectedItems[0].label;
        }

        // If the selected item has "(custom)", trim that part out
        if (selectedType.endsWith(" (custom)")) {
          selectedType = selectedType.substring(
            0,
            selectedType.lastIndexOf(" (custom)")
          );
        }

        // Convert the string into Language
        handleSelection(appManager, selectedType);
        quickPick.dispose();
        resolve();
      });

      quickPick.onDidHide(() => {
        if (!itemSelected) {
          reject("Quick pick was closed");
        }
      });

      quickPick.show();
    });
  }
}

/**
 * Handles the selected or entered language type from the QuickPick interface by converting
 * it to a wasm Language instance and adding it to the application state manager.
 * If an error occurs during this process, it logs the error to the logger.
 *
 * @param appManager - The application state manager instance where the new language will be added.
 * @param selectedType - The label of the selected item or the entered text from the QuickPick interface, representing a programming language.
 */
function handleSelection(
  appManager: appDataManager,
  selectedType: string
): void {
  try {
    const langType = wasm.languageTypeFromFriendlyUX(selectedType);
    const lang = new wasm.Language(langType);
    appManager.addLanguage(lang);
  } catch (error) {
    logger.appendLine(`[ERROR] Failed to add programming language: ${error}`);
  }
}
