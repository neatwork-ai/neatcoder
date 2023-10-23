// ChatContainer.tsx

import React, { useState } from 'react';
import ChatStream from './chatStream';
import TextBox from './textBox';
import { promptLLM } from './httpClient';
import { Message } from '../../wasm/neatcoderInterface';
import QuillEditor from './quil';

let tokenCount;

const ChatContainer: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);

  const handleSendMessage = async (text: string) => {
    const newMessages = [...messages, { user: 'user', ts: "todo", payload: { content: text, role: "user" } }];

    // Add user's message to the chat stream
    setMessages(newMessages);

    // Send message to OpenAI and get response
    try {
      console.log("WebView: Messages: " + JSON.stringify(newMessages));

      const stream = promptLLM(newMessages, true);
      const reader = stream.getReader();

      tokenCount = 0;
      while (true) {
        const { done, value: token } = await reader.read();

        if (token) {
          console.log(token);
          const sanitizedToken = token;
            // .replace(/\n/g, '<br />') // Sanitize new lines
            // .replace(/\*\*(.+?)\*\*/g, '<b>$1</b>'); // Sanitize bold

          try {
            if (tokenCount === 0) {
              setMessages((prevMessages) => [...prevMessages, { user: 'assistant', ts: "todo", payload: { content: sanitizedToken, role: "assistant" } }]);
              tokenCount += 1;
            } else {
              setMessages((prevMessages) => {
                let newMessages = [...prevMessages];

                newMessages[newMessages.length - 1].payload.content += sanitizedToken;
                return newMessages;
              });
            }
          } catch {
            console.error("Unexpected error while streaming")
          }
        }

        if (done) {
          tokenCount += 0;
          break
        };
      }

    } catch (error) {
      console.error("Error getting response from OpenAI:", error);
    }
};

const handleContentChange = (content: any) => {
  console.log(content); // Do something with the content
}


  return (
    <div className="chatContainer">
      <ChatStream className="chatStream" messages={messages} />
      <QuillEditor onContentChange={handleContentChange} />
      <TextBox onSendMessage={handleSendMessage} />
    </div>
  );
};

export default ChatContainer;
