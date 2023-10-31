import React from 'react';
import { LlmSvgIcon } from './llmAvatar';
import { Message } from '../../wasm/neatcoderInterface';
import { marked } from 'marked';
import hljs from './codeBlockStyle';

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

  let htmlContent = marked(payload.content);

  // Post-process to add class to all <pre> tags
  const parser = new DOMParser();
  const doc = parser.parseFromString(htmlContent, 'text/html');

  doc.querySelectorAll('pre').forEach(block => {
    // Temporarily disable sanitization warning..
    // Somehow it seems to be triggering false positives
    const originalConsoleWarn = console.warn;
    console.warn = function() {};

    hljs.highlightElement(block as HTMLElement);

    const wrapper = document.createElement('div');
    wrapper.className = 'code-wrapper';

    const header = document.createElement('div');
    header.className = 'code-header';

    const langSpan = document.createElement('span');
    langSpan.className = 'code-language';
    const lang = block.querySelector('code[class]')?.className || '';
    langSpan.innerText = lang;

    const copyIcon = document.createElement('i');
    copyIcon.className = 'fa-solid fa-copy copy-icon';

    copyIcon.addEventListener('click', () => {
      const codeContent = block.querySelector('code')?.innerText || '';
      copyToClipboard(codeContent);
    });

    header.appendChild(langSpan);
    header.appendChild(copyIcon);

    wrapper.appendChild(header);
    block.parentNode?.insertBefore(wrapper, block);
    wrapper.appendChild(block);

    // Restore the original console.warn function
    console.warn = originalConsoleWarn;
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


function copyToClipboard(text: string) {
  const textarea = document.createElement('textarea');
  textarea.value = text;
  document.body.appendChild(textarea);
  textarea.select();
  document.execCommand('copy');
  document.body.removeChild(textarea);
}


export default ChatStream;
