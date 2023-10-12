import React, { useState, useEffect, useRef } from 'react';
import SVGButton from './sendButton';

const TextBox: React.FC<{ onSendMessage: (text: string) => void }> = ({ onSendMessage }) => {
  const [text, setText] = useState('');

  const handleSend = () => {
    if (text.trim()) {
      onSendMessage(text.trim());
      setText('');
    }
  };

  return (
    <div className="textBox">
      <input
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="Send a message"
      />
      <SVGButton onClick={handleSend} />
    </div>
  );
};

export default TextBox;
