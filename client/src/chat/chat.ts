import * as vscode from "vscode";
import * as fs from "fs";
import * as path from "path";

// export function getWebviewContent() {
//   return `
//         <!DOCTYPE html>
//         <html lang="en">
//         <head>
//             <meta charset="UTF-8">
//             <meta name="viewport" content="width=device-width, initial-scale=1.0">
//             <title>Chat with ChatGPT</title>
//             <style>
//                 body {
//                     font-family: Arial, sans-serif;
//                     padding: 10px;
//                 }
//                 #chatContainer {
//                     border: 1px solid #ccc;
//                     padding: 10px;
//                     height: 400px;
//                     overflow-y: scroll;
//                 }
//                 #inputContainer {
//                     margin-top: 10px;
//                 }
//                 button {
//                     margin-left: 10px;
//                 }
//             </style>
//         </head>
//         <body>
//             <div id="chatContainer">
//                 <!-- Messages will be appended here -->
//             </div>
//             <div id="inputContainer">
//                 <input id="chatInput" type="text" style="width: 80%;">
//                 <button onclick="sendMessage()">Send</button>
//             </div>
//             <script>
//                 function sendMessage() {
//                     const chatInput = document.getElementById('chatInput');
//                     const chatContainer = document.getElementById('chatContainer');

//                     // Append the message to chatContainer
//                     const messageDiv = document.createElement('div');
//                     messageDiv.textContent = 'You: ' + chatInput.value;
//                     chatContainer.appendChild(messageDiv);

//                     // Send message to the VS Code extension
//                     window.vscode.postMessage({
//                         command: 'sendMessage',
//                         text: chatInput.value
//                     });

//                     // Clear the input
//                     chatInput.value = '';
//                 }
//             </script>
//         </body>
//         </html>
//     `;
// }

export function setWebviewContent(
  panel: vscode.WebviewPanel,
  context: vscode.ExtensionContext
) {
  const reactBuildPath = path.join(
    context.extensionPath,
    "..",
    "webview/build/"
  ); // Assuming 'webview' is where you copied your build files
  const entryHtml = path.join(reactBuildPath, "index.html");

  let content = fs.readFileSync(entryHtml, "utf8");

  console.log(`Content before: ${content}`);

  // Use asWebviewUri to get the correct URI for the assets
  const scriptSrc = panel.webview.asWebviewUri(
    vscode.Uri.file(
      path.join(reactBuildPath, "static", "js", "main.684d37d0.js")
    )
  );
  const cssSrc = panel.webview.asWebviewUri(
    vscode.Uri.file(
      path.join(reactBuildPath, "static", "css", "main.073c9b0a.css")
    )
  );

  console.log(scriptSrc);

  // Replace the paths in the HTML content
  content = content.replace(/src="\/static\/js\/(.*?)"/g, (match, filename) => {
    const scriptSrc = panel.webview.asWebviewUri(
      vscode.Uri.file(path.join(reactBuildPath, "static", "js", filename))
    );
    return `src="${scriptSrc}"`;
  });

  content = content.replace(
    /href="\/static\/css\/(.*?)"/g,
    (match, filename) => {
      const cssSrc = panel.webview.asWebviewUri(
        vscode.Uri.file(path.join(reactBuildPath, "static", "css", filename))
      );
      return `href="${cssSrc}"`;
    }
  );

  console.log(`Content after: ${content}`);

  panel.webview.html = content;
}
