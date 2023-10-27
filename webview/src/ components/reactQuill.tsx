// QuillEditor.tsx

import React, { useEffect, useRef } from 'react';
import ReactQuill, { Quill } from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles
import 'font-awesome/css/font-awesome.min.css';
import deltaToMarkdown from '../quillToMarkdown/fromDelta';
import { postProcessCodeBlocks } from '../quillToMarkdown/postProcess';
import { RangeStatic } from 'quill';
import hljs from './codeBlockStyle';
import Parchment from 'parchment';
import { Blot } from 'parchment/dist/typings/blot/abstract/blot';
const Block = Quill.import('blots/block');
const CodeBlock = Quill.import('formats/code-block');



var icons = Quill.import('ui/icons');
// icons['bold'] = '<i class="fa fa-bold" aria-hidden="true"></i>';
// icons['italic'] = '<i class="fa fa-italic" aria-hidden="true"></i>';
// icons['underline'] = '<i class="fa fa-underline" aria-hidden="true"></i>';
// icons['image'] = '<i class="fa fa-picture-o" aria-hidden="true"></i>';
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
        // maxStack: 500,
        // userOnly: true
    },
    // syntax: {
    //     highlight: (text: string) => hljs.highlightAuto(text).value
    // }
};

const formats = [
    // ... other formats we might want to use
    "code",  // inline code
    "code-block",  // code block
];

export const QuillEditor: React.FC<{ onSendMessage: (text: string) => void }> = ({ onSendMessage }) => {
    const [editorContent, setEditorContent] = React.useState('');
    const quillRef = useRef<ReactQuill>(null);

    const handleSend = React.useCallback(() => {
        const delta = quillRef.current?.getEditor().getContents();

        if (delta && delta.ops) {
            console.log("delta: " + JSON.stringify(delta))

            // Join respective code-block lines
            const processedOps = postProcessCodeBlocks(delta.ops);

            // Convert the delta ops to markdown
            const markdownString = deltaToMarkdown(processedOps);

            onSendMessage(markdownString);
            setEditorContent('');  // This will clear the editor
        }
    }, [onSendMessage]);  // Assuming onSendMessage doesn't change often, otherwise add other dependencies


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

            const quillContainer = quill.root;
            quillContainer.addEventListener('keydown', handleKeyDown);

            // Cleanup: remove the event listener when the component is unmounted
            return () => {
                quillContainer.removeEventListener('keydown', handleKeyDown);
            };
        }
    }, [handleSend]);

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
};

export default QuillEditor;

const simulateCodeBlockButtonClick = () => {
    const codeBlockButton = document.querySelector(".ql-code-block");
    if (codeBlockButton) {
        console.log("Clicking...")
        codeBlockButton.dispatchEvent(new Event('click', { 'bubbles': true }));
    }
};
