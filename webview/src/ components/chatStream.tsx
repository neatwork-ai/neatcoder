import React, { CSSProperties, useEffect, useRef, useState } from 'react';
import { LlmSvgIcon } from './llmAvatar';
import { Message } from '../../wasm/neatcoderInterface';
import { marked } from 'marked';
import hljs from './codeBlockStyle';
import ReactMarkdown, {Components} from 'react-markdown';
import SyntaxHighlighter from 'react-syntax-highlighter/dist/esm/prism';
// import { dark } from 'react-syntax-highlighter/dist/esm/styles/prism';
// import { dark, vs2015 } from 'react-syntax-highlighter/dist/esm/styles/hljs';
import codeTheme from './codeTheme';


// // Start with the `dark` theme as the base.
// const baseStyle: { [key: string]: React.CSSProperties } = { ...dark };

// // Define your custom styles for specific tokens.
// const customTokenStyles: { [key: string]: React.CSSProperties } = {
//   'code[class*="language-"]': {
//     ...dark['code[class*="language-"]'], // Spread in existing styles if needed
//     borderRadius: '0.3em',
//     padding: '0.1em',
//     whiteSpace: 'normal',
//   },
//   'comment': {
//     ...dark['comment'], // Spread in existing styles if needed
//     color: 'hsla(0, 0%, 100%, .5)',
//   },
//   'keyword': {
//     ...dark['keyword'], // Spread in existing styles if needed
//     color: '#2e95d3',
//   },
//   'string': {
//     ...dark['string'], // Spread in existing styles if needed
//     color: '#00a67d',
//   },
//   'variable': {
//     ...dark['variable'], // Spread in existing styles if needed
//     color: '#df3079',
//   },
//   // ... add more custom styles for other tokens
// };

// // Create a new style object by merging the base styles with your custom token styles.
// const customStyle: { [key: string]: React.CSSProperties } =  {
//   ...baseStyle,
//   ...customTokenStyles
// };

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

const renderers: Components = {
  // Custom code rendering for fenced code blocks (not inline)
  code({ node, className, children, ...props }) {
    // Check if there's a class indicating the language (e.g., "language-js")
    const match = /language-(\w+)/.exec(className || '');
    if (match) {
      return (
        <SyntaxHighlighter
          language={match[1]}
          style={codeTheme}
        >
          {String(children).replace(/\n$/, '')}
        </SyntaxHighlighter>
      );
    }
    // Return a regular code tag for inline code or unspecified language blocks
    return <code className={className} {...props}>{children}</code>;
  }
};

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
        <ReactMarkdown
          className="custom-pre"
          components={renderers}
        >
            {payload.content}
        </ReactMarkdown>
      </div>
    </div>
  );
};
