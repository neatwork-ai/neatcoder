import React from 'react';

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
  const llmAvatar = `${publicPath}/cyborg.jpg`;

  return (
    <div className={`message ${isUser ? 'user-message' : 'llm-message'}`}>
      <div className="image-container">
        <img src={isUser ? userAvatar : llmAvatar} alt={`${user} profile`} />
      </div>
      <div className="text-container">
        <span className="user-name">{isUser ? 'Username' : 'LLM'}</span>
        <p>{text}</p>
      </div>
    </div>
  );
};

export default ChatStream;
