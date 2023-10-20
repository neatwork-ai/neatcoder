// ChatContainer.tsx

import React, { useState } from 'react';
import ChatStream from './chatStream';
import TextBox from './textBox';
import { buildOpenAIRequest, streamOpenAIResponse } from './httpClient';
import { Message } from '../wasm/neatcoderInterface';

// export interface MessageProps {
//     user: 'bot' | 'user';
//     text: string;
//   }

const ChatContainer: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);

  // const handleSendMessage = (text: string) => {
  //   // Add user's message to the chat stream
  //   setMessages([...messages, { user: 'user', text }]);

  //   // TODO: Send message to OpenAI and get response
  //   // For now, let's just simulate a bot response
  //   setTimeout(() => {
  //     setMessages([...messages, { user: 'user', text }, { user: 'bot', text: "I'm a simulated bot response." }]);
  //   }, 1000);
  // };

  const handleSendMessage = async (text: string) => {
    // Add user's message to the chat stream
    setMessages((prevMessages) => [...prevMessages, { user: 'user', ts: "todo", payload: { content: text, role: "user"} }]);

    // Send message to OpenAI and get response
    try {
      const {apiKey, body} = await buildOpenAIRequest(messages, true);

      // Usage
      const stream = await streamOpenAIResponse(apiKey, body);
      const reader = stream.getReader();

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        console.log(value);  // Here, each value should be a token or whatever unit OpenAI is sending.
      }

      // setMessages((prevMessages) => [...prevMessages, { user: 'bot', text: responseText }]);
    } catch (error) {
      console.error("Error getting response from OpenAI:", error);
      // Optionally display an error message in the chat if desired
      // setMessages((prevMessages) => [...prevMessages, { user: 'bot', text: "Sorry, I couldn't process that request." }]);
    }
  };

  return (
    <div className="chatContainer">
      <ChatStream className="chatStream" messages={messages} />
      <TextBox onSendMessage={handleSendMessage} />
    </div>
  );
};

export default ChatContainer;
