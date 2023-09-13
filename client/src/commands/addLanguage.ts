import * as vscode from "vscode";
import { langList } from "../models/mappings";
import { AppStateManager } from "../appStateManager";
import * as wasm from "../../pkg/neatcoder";

export function addLanguage(
  appManager: AppStateManager,
  logger: vscode.OutputChannel
): Promise<void> {
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
        handleSelection(appManager, selectedType, logger);
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

function handleSelection(
  appManager: AppStateManager,
  selectedType: string,
  logger: vscode.OutputChannel
): void {
  try {
    const langType = wasm.languageTypeFromFriendlyUX(selectedType);
    const lang = new wasm.Language(langType);
    appManager.addLanguage(lang);
  } catch (error) {
    logger.appendLine(`[INFO] Failed to add programming language: ${error}`);
  }
}
