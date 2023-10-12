// ChatContainer.tsx

import React, { useState } from 'react';
import ChatStream from './chatStream';
import TextBox from './textBox';

export interface MessageProps {
    user: 'bot' | 'user';
    text: string;
  }

const ChatContainer: React.FC = () => {
  const [messages, setMessages] = useState<MessageProps[]>([]);

  const handleSendMessage = (text: string) => {
    // Add user's message to the chat stream
    setMessages([...messages, { user: 'user', text }]);

    // TODO: Send message to OpenAI and get response
    // For now, let's just simulate a bot response
    setTimeout(() => {
      setMessages([...messages, { user: 'user', text }, { user: 'bot', text: "I'm a simulated bot response." }]);
    }, 1000);
  };

  return (
    <div className="chatContainer">
      <ChatStream className="chatStream" messages={messages} />
      <TextBox onSendMessage={handleSendMessage} />
    </div>
  );
};

export default ChatContainer;
