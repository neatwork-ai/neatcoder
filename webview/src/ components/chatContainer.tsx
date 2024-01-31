// ChatContainer.tsx

import React, { useEffect, useRef, useState } from 'react';
import ChatStream from './chatStream';
import { promptLLM, saveChat } from './vsceClient';
import { MessageDataWasm as Message } from '../../wasm/neatcoderInterface';
import QuillEditor from './reactQuill';
import SendButton from './sendButton';

let tokenCount;

const ChatContainer: React.FC = () => {
  // Initialize messages from window.initialChatHistory if it exists
  const [messages, setMessages] = useState<Message[]>(
    window.initialChatHistory || []
  );

  const quillRef = useRef<any>(null);
  const [isStreaming, setIsStreaming] = useState(false);  // State to track if streaming is active
  const [canSave, setCanSave] = useState(false);

  const handleSendMessage = async (text: string) => {
    if (isStreaming) {
      // Prevent sending new messages if stream is active
      console.warn("Please wait for the current response to finish streaming.");
      return;
    }

    let localMessages = [...messages, { user: 'user', ts: new Date(), payload: { content: text, role: "user" } }];

    // Add user's message to the chat stream
    setMessages(localMessages);

    // Send message to OpenAI and get response
    try {
      setIsStreaming(true); // Start streaming
      const stream = promptLLM(localMessages, true);
      const reader = stream.getReader();

      tokenCount = 0;
      while (true) {
        const { done, value: token } = await reader.read();

        if (token) {
          try {
            if (tokenCount === 0) {
              const newAssistantMessage = { user: 'assistant', ts: new Date(), payload: { content: token, role: "assistant" } };
              setMessages((prevMessages) => [...prevMessages, newAssistantMessage]);
              tokenCount += 1;
            } else {
              setMessages((prevMessages) => {
                // Create a new object for the last message with updated content
                const lastMessageIndex = prevMessages.length - 1;
                const updatedLastMessage = {
                  ...prevMessages[lastMessageIndex],
                  payload: {
                    ...prevMessages[lastMessageIndex].payload,
                    content: prevMessages[lastMessageIndex].payload.content + token
                  }
                };

                // Create a new array with all previous messages except the last one
                const newMessages = prevMessages.slice(0, lastMessageIndex);
                // Add the updated last message to the array
                newMessages.push(updatedLastMessage);
                return newMessages;
              });
            }
          } catch {
            console.error("Unexpected error while streaming")
          }
        }

        if (done) {
          setCanSave(true);
          setIsStreaming(false); // End streaming
          tokenCount += 0;
          break
        };
      }

    } catch (err) {
      console.error("Error getting response from OpenAI:", err);
      throw new Error((err as Error).message);
    }
  };

  const handleSendButtonClick = () => {
    if (quillRef.current) {
      quillRef.current.handleSend();
    }
  };

  useEffect(() => {
    // This effect runs when `canSave` changes.
    // When canSave is set to true, you can save the chat.
    if (canSave) {
      saveChat(messages);
      setCanSave(false);
    }
  }, [canSave, messages]);


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
