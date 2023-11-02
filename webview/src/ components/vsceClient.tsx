import { Message } from "../../wasm/neatcoderInterface";

const vscode = acquireVsCodeApi();

export function promptLLM(msgs: Array<Message>, stream: boolean): ReadableStream {
  const { readable, writable } = new TransformStream();

  const writer = writable.getWriter();

  vscode.postMessage({
    command: 'promptLLM',
    msgs: msgs,
    stream: stream
  });

  function handleMessage(event: MessageEvent<any>) {
    const message = event.data;

    if (message.command === 'dataChunk') {
      writer.write(message.chunk);
    } else if (message.command === 'endOfStream') {
      writer.close();
      // Remove the event listener after handling 'endOfStream'
      window.removeEventListener('message', handleMessage);
    }
  }

  window.addEventListener('message', handleMessage);

  return readable;
}

export function saveChat(msgs: Array<Message>) {
  vscode.postMessage({
    command: 'saveChat',
    msgs: msgs,
  });
}
