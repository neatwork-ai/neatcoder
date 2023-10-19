// async function handleOpenAIRequest(message: any) {
//   const msgs = message.msgs;
//   const stream = message.stream;

//   // Do whatever processing you need to generate the apiKey and body
//   // For the sake of this example, I'll assume you have some function called
//   // generateApiKeyAndBody() that returns the required data

//   const { apiKey, body } = await generateApiKeyAndBody(msgs, stream);

//   // Respond back to the webview
//   webviewPanel.webview.postMessage({
//     command: "buildOpenAIRequest",
//     apiKey: apiKey,
//     body: body,
//   });
// }
