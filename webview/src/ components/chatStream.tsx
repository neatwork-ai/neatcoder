import React from 'react';
import { LlmSvgIcon } from './llmAvatar';
import { Message } from '../../wasm/neatcoderInterface';
import { marked } from 'marked';

const renderer = new marked.Renderer();

marked.setOptions({ renderer });

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

  // const renderer = new marked.Renderer();
  // // Override the default behavior for 'pre' elements
  // renderer.pre = (code, infoString, escaped) => {
  //   return `<pre class="custom-pre">${code}</pre>`;
  // };

  // marked.setOptions({ renderer });

  let htmlContent = marked(payload.content);

  // Post-process to add class to all <pre> tags
  const parser = new DOMParser();
  const doc = parser.parseFromString(htmlContent, 'text/html');
  doc.querySelectorAll('pre').forEach((pre) => {
    pre.classList.add('custom-pre');
  });
  htmlContent = doc.body.innerHTML;

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
        <pre className="custom-pre" dangerouslySetInnerHTML={{ __html: htmlContent }} />
      </div>
    </div>
  );
};

export default ChatStream;

// Type guard function
function isJSXElementArray(content: string | JSX.Element[]): content is JSX.Element[] {
  return Array.isArray(content);
}
