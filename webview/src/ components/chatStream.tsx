import React from 'react';
import { LlmSvgIcon } from './llmAvatar';
import { Message } from '../../wasm/neatcoderInterface';

interface ChatStreamProps {
  messages: Message[];
  className?: string;
}

const ChatStream: React.FC<ChatStreamProps> = ({ messages, className }) => (
  <div className={className}>
    {messages.map((message, idx) => <MessageUi key={idx} {...message} />)}
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
        <pre className="custom-pre">
          {isJSXElementArray(payload.content)
            ? payload.content.map((elem, idx) => <React.Fragment key={idx}>{elem}</React.Fragment>)
            : <span dangerouslySetInnerHTML={{ __html: payload.content }} />
          }
        </pre>
      </div>
    </div>
  );
};

export default ChatStream;

// Type guard function
function isJSXElementArray(content: string | JSX.Element[]): content is JSX.Element[] {
  return Array.isArray(content);
}
