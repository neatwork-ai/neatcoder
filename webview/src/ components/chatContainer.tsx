// ChatContainer.tsx

import React, { useRef, useState } from 'react';
import ChatStream from './chatStream';
import { promptLLM } from './httpClient';
import { Message } from '../../wasm/neatcoderInterface';
import QuillEditor from './reactQuill';
import SendButton from './sendButton';

let tokenCount;

const ChatContainer: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);
  const quillRef = useRef<any>(null);
  const [isStreaming, setIsStreaming] = useState(false);  // State to track if streaming is active

  const handleSendMessage = async (text: string) => {
    if (isStreaming) {
      // Prevent sending new messages if stream is active
      console.warn("Please wait for the current response to finish streaming.");
      return;
    }

    const newMessages = [...messages, { user: 'user', ts: new Date(), payload: { content: text, role: "user" } }];

    // Add user's message to the chat stream
    setMessages(newMessages);

    // Send message to OpenAI and get response
    try {
      setIsStreaming(true); // Start streaming
      const stream = promptLLM(newMessages, true);
      const reader = stream.getReader();

      tokenCount = 0;
      while (true) {
        const { done, value: token } = await reader.read();

        if (token) {
          try {
            if (tokenCount === 0) {
              setMessages((prevMessages) => [...prevMessages, { user: 'assistant', ts: new Date(), payload: { content: token, role: "assistant" } }]);
              tokenCount += 1;
            } else {
              setMessages((prevMessages) => {
                let newMessages = [...prevMessages];

                newMessages[newMessages.length - 1].payload.content += token;
                return newMessages;
              });
            }
          } catch {
            console.error("Unexpected error while streaming")
          }
        }

        if (done) {
          setIsStreaming(false); // End streaming
          tokenCount += 0;
          break
        };
      }

    } catch (error) {
      console.error("Error getting response from OpenAI:", error);
    }
  };

  const handleSendButtonClick = () => {
    if (quillRef.current) {
      quillRef.current.handleSend();
    }
  };


  return (
    <div className="chatContainer">
      <ChatStream className="chatStream" messages={messages} />
      <div className= "input-wrapper">
        <SendButton onClick={handleSendButtonClick} disabled={isStreaming}/>
        <QuillEditor ref={quillRef} onSendMessage={handleSendMessage} isStreaming={isStreaming}/>
      </div>
    </div>
  );
};

export default ChatContainer;
