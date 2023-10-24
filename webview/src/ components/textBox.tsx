import React, { useState, useEffect, useRef } from 'react';
import SVGButton from './sendButton';

const TextBox: React.FC<{ onSendMessage: (text: string) => void }> = ({ onSendMessage }) => {
  const [text, setText] = useState('');
  const [rawText, setRawText] = useState('');  // Store the original unformatted text
  const textDivRef = useRef<HTMLDivElement | null>(null);

  const handleSend = () => {
    if (rawText.trim()) {
      onSendMessage(rawText.trim());
      setText('');
      setRawText('');  // Reset the original text too
    }
  };

  const handleKeyPress = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
      event.preventDefault();
      handleSend();
    }
  };

  const handleChange = () => {
    if (textDivRef.current) {
      const currentPos = saveCaretPosition(textDivRef.current); // Save current position

      const content = textDivRef.current.innerText;  // Get the plain text
      setRawText(content);  // Update the original text

      // Formatting for the display within the div
      const formattedContent = content
        .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')  // for bold
        .replace(/`(.*?)`/g, '<span class="one-liner-code">$1</span>')  // for inline code
        .replace(/```(.*?)```/g, '<div class="code-block">$1</div>');  // for code block

      setText(formattedContent);
      textDivRef.current.innerHTML = formattedContent;

      restoreCaretPosition(textDivRef.current, currentPos); // Restore position
    }
  };

  return (
    <div className="textBox">
        <div
            ref={textDivRef}
            contentEditable={true}
            onInput={handleChange}
            onKeyDown={handleKeyPress}
            placeholder="Send a message"
            className="editable-div"
        ></div>
        <SVGButton onClick={handleSend} />
    </div>
  );
};

const saveCaretPosition = (context: HTMLElement): number => {
  const selection = window.getSelection();
  if (selection && selection.rangeCount > 0) {
    const range = selection.getRangeAt(0);
    range.setStart(context, 0);
    const len = range.toString().length;
    return len;
  }
  return 0;
}

const restoreCaretPosition = (context: HTMLElement, pos: number): void => {
  const selection = window.getSelection();
  if (selection) {
    const range = document.createRange();
    if (context.firstChild) {
      range.setStart(context.firstChild, pos);
      range.collapse(true);
      selection.removeAllRanges();
      selection.addRange(range);
    }
  }
}


export default TextBox;
