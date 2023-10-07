export function getWebviewContent() {
  return `
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Chat with ChatGPT</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    padding: 10px;
                }
                #chatContainer {
                    border: 1px solid #ccc;
                    padding: 10px;
                    height: 400px;
                    overflow-y: scroll;
                }
                #inputContainer {
                    margin-top: 10px;
                }
                button {
                    margin-left: 10px;
                }
            </style>
        </head>
        <body>
            <div id="chatContainer">
                <!-- Messages will be appended here -->
            </div>
            <div id="inputContainer">
                <input id="chatInput" type="text" style="width: 80%;">
                <button onclick="sendMessage()">Send</button>
            </div>
            <script>
                function sendMessage() {
                    const chatInput = document.getElementById('chatInput');
                    const chatContainer = document.getElementById('chatContainer');

                    // Append the message to chatContainer
                    const messageDiv = document.createElement('div');
                    messageDiv.textContent = 'You: ' + chatInput.value;
                    chatContainer.appendChild(messageDiv);

                    // Send message to the VS Code extension
                    window.vscode.postMessage({
                        command: 'sendMessage',
                        text: chatInput.value
                    });

                    // Clear the input
                    chatInput.value = '';
                }
            </script>
        </body>
        </html>
    `;
}
