import React from 'react';

interface MessageProps {
  user: 'bot' | 'user';
  text: string;
}

const ChatStream: React.FC<{ messages: MessageProps[] }> = ({ messages }) => (
  <div className="chatStream">
    {messages.map((message, index) => (
      <Message key={index} {...message} />
    ))}
  </div>
);

const Message: React.FC<MessageProps> = ({ user, text }) => (
  <div className={`message ${user === 'bot' ? 'bot' : 'user'}`}>
    <p>{text}</p>
  </div>
);

export default ChatStream;
