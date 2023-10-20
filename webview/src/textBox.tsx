import React, { useState, useEffect, useRef } from 'react';
import SVGButton from './sendButton';

const TextBox: React.FC<{ onSendMessage: (text: string) => void }> = ({ onSendMessage }) => {
  const [text, setText] = useState('');
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);

  const handleSend = () => {
    if (text.trim()) {
      onSendMessage(text.trim());
      setText('');
    }
  };

  const handleKeyPress = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
      event.preventDefault();  // Prevents the default behavior of the Enter key (i.e., a newline)
      handleSend();
    }
  };

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = '2px';
      const computed = window.getComputedStyle(textareaRef.current);
      const height = parseInt(computed.getPropertyValue('border-top-width'), 10)
          + parseInt(computed.getPropertyValue('border-bottom-width'), 10)
          + textareaRef.current.scrollHeight;
      textareaRef.current.style.height = `${height}px`;
    }
  }, [text]);

  return (
    <div className="textBox">
        <textarea
            ref={textareaRef}
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyPress={handleKeyPress}  // Add this line
            placeholder="Send a message"
        />
        <SVGButton onClick={handleSend} />
    </div>
);
};

export default TextBox;
