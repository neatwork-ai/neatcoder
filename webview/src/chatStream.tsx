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

const Message: React.FC<MessageProps> = ({ user, text }) => {
  const isUser = user === 'user';
  const publicPath = (window as any).publicPath;

  const userAvatar = `${publicPath}/default_user.jpg`;
  const llmAvatar = `${publicPath}/cyborg.jpg`;

  return (
    <div className={`message ${isUser ? 'user-message' : 'llm-message'}`}>
      {/* <img src={isUser ? imagePath : llmAvatar} alt={`${user} profile`} /> */}
      <img src={isUser ? userAvatar : llmAvatar} alt={`${user} profile`} />
      <span className="user-name">{isUser ? 'Username' : 'LLM'}</span>
      <p>{text}</p>
    </div>
  );
};

export default ChatStream;
