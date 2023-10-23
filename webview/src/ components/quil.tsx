import React, { useEffect, useRef } from 'react';
import Quill from 'quill';
import SVGButton from './sendButton';

// Define Custom Inline Blot for Inline Code, as previously discussed

let Inline = Quill.import('blots/inline');
class InlineCodeBlot extends Inline { }
InlineCodeBlot.blotName = 'inlineCode';
InlineCodeBlot.className = 'one-liner-code';
InlineCodeBlot.tagName = 'span';

Quill.register(InlineCodeBlot);

const QuillEditor: React.FC<{ onContentChange: (content: any) => void }> = ({ onContentChange }) => {
  const editorRef = useRef<HTMLDivElement | null>(null);
  const quillInstance = useRef<Quill | null>(null);

  useEffect(() => {
      if (editorRef.current) {
        quillInstance.current = new Quill(editorRef.current, {
            modules: {
                // Any additional modules you might want to add
            },
            placeholder: 'Start typing...',

        });

        quillInstance.current.on('text-change', function(delta, oldDelta, source) {
            if (source === 'user') {
                // Handle the text change event if needed
            }
        });
      }
    },
  []);


  return (
    <div className="textBox">
        <div ref={editorRef}></div>
      <SVGButton onClick={() => {}} />
    </div>
  );
}

export default QuillEditor;
