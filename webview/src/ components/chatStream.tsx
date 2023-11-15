import React, { CSSProperties, useEffect, useRef, useState } from 'react';
import { LlmSvgIcon } from './llmAvatar';
import { MessageDataWasm as Message } from '../../wasm/neatcoderInterface';
import hljs from './codeBlockStyle';
import ReactMarkdown from 'react-markdown';

interface ChatStreamProps {
  messages: Message[];
  className?: string;
}

const ChatStream: React.FC<ChatStreamProps> = ({ messages, className }) => {
  return (<div className={className}>
    {messages.map((message, idx) => <MessageUi key={idx} {...message} />)}
  </div>
)};

export default ChatStream;

function copyToClipboard(text: string) {
  // Modern async clipboard API needs to be inside a user-triggered event
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text).then(
      () => {
        // Handle successful copying
        console.log('Text copied to clipboard');
      },
      (err) => {
        // Handle errors here
        console.error('Failed to copy: ', err);
      }
    );
  } else {
    // Fallback for older browsers
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';  // Prevent scrolling to bottom of page in MS Edge.
    document.body.appendChild(textarea);
    textarea.focus(); // Focus on the textarea to make sure the copy command works
    textarea.select();
    try {
      const successful = document.execCommand('copy');
      const msg = successful ? 'successful' : 'unsuccessful';
      console.log('Fallback: Copying text command was ' + msg);
    } catch (err) {
      console.error('Fallback: Oops, unable to copy', err);
    }
    document.body.removeChild(textarea);
  }
}

const MessageUi: React.FC<Message> = ({ user, ts, payload }) => {
  const isUser = user === 'user';
  const publicPath = (window as any).publicPath;
  const userAvatar = `${publicPath}/default_user.jpg`;
  const markdownRef = useRef<HTMLDivElement>(null);

  // TODO: add a code-padding component...
  useEffect(() => {
      // Highlight all code blocks within the markdownRef current element
      markdownRef.current?.querySelectorAll<HTMLElement>(':scope > .custom-pre > pre code').forEach((block) => {
        // Assuming codeElement is the <code> inside the <pre>
        // Add 'code-wrapper' class to the parent <pre> of this <code>
        const preElement = block.parentElement;
        if (preElement) {
          preElement.classList.add('code-wrapper');
        }

        // Remove the attribute to force re-highlighting
        delete block.dataset.highlighted;

        hljs.highlightElement(block as HTMLElement);

        if (preElement && !preElement.dataset.withHead) {
          const codeHeader = document.createElement('div');
          codeHeader.className = 'code-header';

          const langSpan = document.createElement('span');
          langSpan.className = 'code-language';
          const lang = block.querySelector('code[class]')?.className || '';
          langSpan.innerText = lang;

          const copyButton = document.createElement('button');
          copyButton.className = 'fa-solid fa-copy copy-icon';
          copyButton.onclick = () => {
            // Assuming the `copyToClipboard` function takes the text you want to copy as an argument
            if (block.textContent) {
                copyToClipboard(block.textContent);
            }
          };

          codeHeader.appendChild(langSpan);
          codeHeader.appendChild(copyButton);

          preElement.prepend(codeHeader);

          preElement.dataset.withHead = "yes";
        }
      });

  }, [payload]); // Re-run the effect when payload changes

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
        <div className="markdown-wrapper" ref={markdownRef}>
          <ReactMarkdown
            className="custom-pre"
          >
              {payload.content}
          </ReactMarkdown>
        </div>
      </div>
    </div>
  );
};
