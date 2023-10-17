// /// <reference types="./vscode.d.ts" />

export function buildOpenAIRequest(text: string): Promise<string> {
    return new Promise((resolve) => {
      // Post the message to the VS Code extension
      (window as any).vscode.postMessage({
        command: 'buildOpenAIRequest',
        text: text
      });

      function handleVscodeMessage(event: any) {
        const message = event.data;

        if (message.command === 'buildOpenAIRequest') {
          window.removeEventListener('message', handleVscodeMessage);
          resolve(message.text);
        }
      }

      window.addEventListener('message', handleVscodeMessage);
    });
  }
