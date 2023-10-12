import React from 'react';
import { LlmSvgIcon } from './llmAvatar';

interface MessageProps {
  user: 'bot' | 'user';
  text: string;
}

interface ChatStreamProps {
  messages: MessageProps[];
  className?: string;
}

const ChatStream: React.FC<ChatStreamProps> = ({ messages, className }) => (
  <div className={className}>
    {messages.map((message, index) => (
      <Message key={index} {...message} />
    ))}
  </div>
);

const Message: React.FC<MessageProps> = ({ user, text }) => {
  const isUser = user === 'user';
  const publicPath = (window as any).publicPath;

  const userAvatar = `${publicPath}/default_user.jpg`;

  return (
    <div className={`message ${isUser ? 'user-message' : 'llm-message'}`}>
      <div className="image-container">
      {isUser ? (
          <img src={userAvatar} alt="user profile" />
        ) : (
          <LlmSvgIcon />
        )}
      </div>
      <div className="text-container">
        <span className="user-name">{isUser ? 'User' : 'Neatcoder'}</span>
        <p>{text}</p>
      </div>
    </div>
  );
};

export default ChatStream;
