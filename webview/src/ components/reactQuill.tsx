// QuillEditor.tsx

import React, { forwardRef, useEffect, useImperativeHandle, useRef } from 'react';
import ReactQuill, { Quill } from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles
import 'font-awesome/css/font-awesome.min.css';
import { RangeStatic } from 'quill';
import TurndownService from 'turndown';

let currentSelection: RangeStatic | null = null;

// Initialize Turndown service
const turndownService = new TurndownService();

// Disable all escaping - TODO: If we introduce markdown rendering this
// will become a problem
turndownService.escape = (string: string) => string;

// Define a custom rule for 'pre.ql-syntax' elements
turndownService.addRule('qlSyntax', {
    filter: (node) => {
      return (
        node.nodeName === 'PRE' &&
        node.classList.contains('ql-syntax')
      );
    },
    replacement: (content) => {
      // The replacement function will wrap the content in a code block syntax
      return '\n```\n' + content.trim() + '\n```\n';
    }
  });

var icons = Quill.import('ui/icons');
icons['code'] = '<i class="fa fa-code" aria-hidden="true"></i>';
icons['code-block'] = '<i class="fas fa-file-code-o" aria-hidden="true"></i>';

const CustomToolbar: React.FC = () => {
    return (
        <div id="toolbar">
            <button className="ql-code"/>
            <button className="ql-code-block"/>
        </div>
    );
};

const modules = {
    toolbar: {
        container: "#toolbar",
    },
    history: {
        delay: 20,
    },
};

const formats = [
    // ... other formats we might want to use
    "code",  // inline code
    "code-block",  // code block
];

type QuillEditorProps = {
    onSendMessage: (text: string) => void;
  };

  // You will also need to define the type for the ref
  // This will be the type of the object you're exposing, which contains the handleSend method
  type QuillEditorHandles = {
    handleSend: () => void;
  };


export const QuillEditor = forwardRef<QuillEditorHandles, QuillEditorProps>(
    ({ onSendMessage }, ref) => {
        const [editorContent, setEditorContent] = React.useState('');
        const quillRef = useRef<ReactQuill>(null);

        const handleSend = React.useCallback(() => {
            // Assuming quillRef is a ref to the Quill editor instance
            const editor = quillRef.current?.getEditor();

            if (editor) {
                const htmlContent = editor.root.innerHTML; // Get the inner HTML content
                console.log("HTML Content: " + htmlContent);

                // Convert the HTML to Markdown
                const markdownString = turndownService.turndown(htmlContent);
                console.log("Markdown String: " + markdownString);

                // Assuming onSendMessage is a prop that handles sending the markdown content
                onSendMessage(markdownString);
                editor.setText('');  // This will clear the editor
            }
        }, [onSendMessage, quillRef]); // Add quillRef to dependencies if it changes

        useEffect(() => {
            if (quillRef.current) {
                const quill = quillRef.current.getEditor();

                // Disable default behaviour of `Enter`
                const keyboard = quill.getModule('keyboard');
                keyboard.bindings['Enter'] = null;
                keyboard.bindings['13'] = null;

                let consecutivePresses = 0;

                keyboard.addBinding({
                    key: 'I',
                    shiftKey: true,
                    shortKey: true,
                }, (range: RangeStatic, context: KeyboardEvent) => {
                    consecutivePresses += 1;
                    const format = quill.getFormat(range);

                    // If already in code block, then exit code-block
                    if (format['code-block']) {
                        quill.format('code-block', false);
                        consecutivePresses = 0;
                        return;
                    }

                    // If two consecutive presses then activate code-block
                    if (consecutivePresses === 2) {
                        const rangeCopy = { ...range };

                        quill.format('code', false); // remove inline code

                        // For some reason the state of the range seems to
                        // change somewhere under the hood, so we make a deep copy
                        if (rangeCopy) {
                            setTimeout(() => {
                                console.log("Applying format to " + JSON.stringify(rangeCopy))
                                quill.formatLine(rangeCopy.index, rangeCopy.length, 'code-block', true);
                            }, 25);
                        }

                        consecutivePresses = 0;
                        return;
                    }

                    // If inside inline-code but not consecutive press then
                    // exit inline-code
                    if (format.code) {
                        quill.format('code', false); // exit iniline
                        consecutivePresses = 0;
                    } else { // If not inside inline-code then activate inline-code
                        quill.format('code', true); // activate inline
                    }

                    return;
                });

                const handleKeyDown = (event: KeyboardEvent) => {
                    // The Quill Editor Undo functionality is messed up
                    // in the iOS (as usual, what is not messed up with Quill?)
                    // So we Check for undo (Ctrl + Z / Cmd + Z) and prevent the
                    // default behaviour.
                    //
                    // The intention was to add a custom binding, but somehow,
                    // placing the handler at the beginning of the event handler
                    // and triggering the immediate stop of the propagation just
                    // makes it work...
                    if ((event.ctrlKey || event.metaKey) && event.key === "z") {
                        console.log('Custom undo action');
                        console.log('is cancellable?', event.cancelable);
                        event.preventDefault();
                        event.stopImmediatePropagation();
                        event.stopPropagation();
                        return false;
                    }

                    // This allows us to click enter to send the message to openAI
                    if (event.key === 'Enter' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
                        handleSend();
                        return
                    }

                    if (event.key === 'Enter' && event.shiftKey) {
                        const selection = quill.getSelection();
                        const format = quill.getFormat(selection!);

                        if (format.code) {
                            console.log("OINC")
                            // If inside inline code, prevent the newline
                            event.preventDefault();
                            const startIndex = selection!.index;

                            // Checking if the next character after the selection doesn't have the inline code format
                            const formatAfterCursor = quill.getFormat(startIndex + 1, 1);

                            // If inside inline code and in the middle of the code,
                            // then prevent the newline, else go ahead and allow newline
                            if (!formatAfterCursor.code) {
                                quill.format('code', false); // remove the code format
                                quill.insertText(startIndex + 1, '\n', {}, 'user'); // insert a newline
                                quill.setSelection(startIndex + 2, 0); // set the cursor after the newline
                            }

                            return;
                        }

                        event.preventDefault();  // prevent the default behavior
                        const currentSelection = quill.getSelection();
                        if (currentSelection) {
                            const currentPosition = currentSelection.index;
                            quill.insertText(currentPosition, '\n', 'user');
                            quill.setSelection(currentPosition + 1, 0, 'user'); // set the cursor just after the newline
                        }

                        return;
                    }

                    if (event.key !== 'I' && event.key !== 'Meta' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
                        // Reset presses
                        consecutivePresses = 0;
                    }
                };

                quill.on('selection-change', (range, oldRange, source) => {
                    if (range) {
                        currentSelection = range;
                    }
                });

                // If the content being pasted comes from vscode then
                // we intercept the pasting and apply code-block format
                const handlePaste = (event: ClipboardEvent) => {
                    if (event.clipboardData) {
                        const types = event.clipboardData.types;
                        if (types.includes('vscode-editor-data')) {
                            // Get the current selection
                            const selection = currentSelection;

                            if (selection) {
                                // Create an empty code block at the current position
                                console.log("NEWLINE!")
                                quill.insertText(selection.index, '\n', { 'code-block': true });

                                console.log("MOVE SELECTION!")
                                // Move the cursor inside the code block
                                // quill.setSelection(selection!.index + 1, 1);
                                console.log("Selection moved!")

                            }

                            // Let the default paste behavior occur
                        }
                    }
                };


                const quillContainer = quill.root;
                quillContainer.addEventListener('keydown', handleKeyDown);
                quillContainer.addEventListener('paste', handlePaste);

                // Cleanup: remove the event listener when the component is unmounted
                return () => {
                    quillContainer.removeEventListener('keydown', handleKeyDown);
                    quillContainer.removeEventListener('paste', handlePaste);
                };
            }
        }, [handleSend]);

      // Expose the handleSend method to the parent component using useImperativeHandle
      useImperativeHandle(ref, () => ({
        handleSend,
      }));

      // Don't forget to return your component's JSX!
      return (
        <div>
            <CustomToolbar />
            <div className='ql-container-decorator'>
                <ReactQuill
                    ref={quillRef}
                    value={editorContent}
                    onChange={setEditorContent}
                    modules={modules}
                    formats={formats}
                />
            </div>

        </div>
    );
    }
);

export default QuillEditor;
