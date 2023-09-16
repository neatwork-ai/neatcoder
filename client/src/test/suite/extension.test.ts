import * as assert from "assert";
import * as wasm from "../../../pkg/neatcoder";

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
import * as vscode from "vscode";

suite("Extension Test Suite", () => {
  vscode.window.showInformationMessage("Start all tests.");

  test("Sample test", () => {
    assert.strictEqual(-1, [1, 2, 3].indexOf(5));
    assert.strictEqual(-1, [1, 2, 3].indexOf(0));
  });

  test("Test appData Serialization/Deserialization", () => {
    const stateData = {
      specs: null,
      scaffold: null,
      interfaces: {},
      taskPool: {
        counter: 0,
        todo: {
          tasks: {},
          order: [],
        },
        done: {
          tasks: {},
          order: [],
        },
      },
    };
    const json = JSON.stringify(stateData, null, 4);
    const appState = wasm.AppState.castFromString(json);
    console.log(appState);
  });
});
