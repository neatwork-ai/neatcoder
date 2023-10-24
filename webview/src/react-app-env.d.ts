/// <reference types="react-scripts" />

interface VsCodeApi {
  postMessage(message: any): void;
}

// This function is injected by the VS Code runtime as
// a way to expose the VS Code api
declare function acquireVsCodeApi(): VsCodeApi;
