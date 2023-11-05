import React, { useEffect, useRef, useState } from 'react';
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

const ChatStream: React.FC<ChatStreamProps> = ({ messages, className }) => {
  return (<div className={className}>
    {messages.map((message, idx) => <MessageUi key={idx} {...message} />)}
  </div>
)};

const MessageUi: React.FC<Message> = ({ user, ts, payload }) => {
  const isUser = user === 'user';
  const publicPath = (window as any).publicPath;
  const userAvatar = `${publicPath}/default_user.jpg`;
  const [htmlContent, setHtmlContent] = useState('');
  // Typing the ref with HTMLDivElement because the ref will be attached to a div element

  useEffect(() => {
    // Ensure the payload content is present
    if (payload && payload.content) {
      // Convert markdown to HTML
      const rawHtml = marked(payload.content);
      // Create a virtual DOM element to manipulate
      const virtualDocument = new DOMParser().parseFromString(rawHtml, 'text/html');

      // Set the innerHTML directly on the ref's current element
      // Apply syntax highlighting and other transformations
      virtualDocument.querySelectorAll('pre').forEach(block => {

        // Apply syntax highlighting to all <pre> elements
        hljs.highlightElement(block as HTMLElement);

        const wrapper = document.createElement('div');
        wrapper.className = 'code-wrapper';

        const codeHeader = document.createElement('div');
        codeHeader.className = 'code-header';

        const langSpan = document.createElement('span');
        langSpan.className = 'code-language';
        const lang = block.querySelector('code[class]')?.className || '';
        langSpan.innerText = lang;

        const copyButton = document.createElement('button');
        copyButton.className = 'fa-solid fa-copy copy-icon';
        copyButton.onclick = () => {
          console.log("copy");
          // Assuming the `copyToClipboard` function takes the text you want to copy as an argument
          if (block.textContent) {
              copyToClipboard(block.textContent);
          }
        };

        codeHeader.appendChild(langSpan);
        codeHeader.appendChild(copyButton);

        wrapper.appendChild(codeHeader);
        block.parentNode?.insertBefore(wrapper, block);
        wrapper.appendChild(block);
      });

      // Set the HTML content to the transformed HTML
      setHtmlContent(virtualDocument.body.innerHTML);

    }
  }, [payload]);

  return (
    <div className={`message ${isUser ? 'user-message' : 'llm-message'}`}>
      <div className="image-container">
        {isUser ? (
          <img src={userAvatar} alt="user profile" />
        ) : (
          // Replace LlmSvgIcon with actual SVG or component
          <LlmSvgIcon />
        )}
      </div>
      <div className="text-container">
        <span className="user-name">{isUser ? 'User' : 'Neatcoder'}</span>
        <div
            className="custom-pre"
            dangerouslySetInnerHTML={{ __html: htmlContent }}
        />
      </div>
    </div>
  );
};

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
