import React from 'react';
import { LlmSvgIcon } from './llmAvatar';
import { Message } from '../wasm/neatcoderInterface';

interface ChatStreamProps {
  messages: Message[];
  className?: string;
}

const ChatStream: React.FC<ChatStreamProps> = ({ messages, className }) => (
  <div className={className}>
    {messages.map((message, index) => (
      <MessageUi key={index} {...message} />
    ))}
  </div>
);

const MessageUi: React.FC<Message> = ({ user, ts, payload }) => {
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
        <p>{payload.content}</p>
      </div>
    </div>
  );
};

export default ChatStream;
