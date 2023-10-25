// QuillEditor.tsx

import React, { useEffect, useRef } from 'react';
import ReactQuill, { Quill } from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles
import 'font-awesome/css/font-awesome.min.css';
import deltaToMarkdown from '../quillToMarkdown/fromDelta';

const delayLoop = (iterations: number) => {
    for (let i = 0; i < iterations; i++) { }
};


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
            // Convert the delta ops to markdown
            console.log(`delta: ${JSON.stringify(delta)}`)
            const markdownString = deltaToMarkdown(delta.ops);
            console.log(`markdownString: ${markdownString}`)

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

            let consecutiveBackticks = 0;  // Track consecutive backtick presses

            const handleKeyDown = (event: KeyboardEvent) => {
                if (event.key === 'Enter' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
                    handleSend();
                }

                if (event.code === "BracketRight") {  // Key code for backtick
                    consecutiveBackticks++;
                    console.log("consecutiveBackticks: " + consecutiveBackticks);

                    if (consecutiveBackticks === 3) {
                        console.log("Three backticks!");
                        const currentSelection = quill.getSelection(); // Get current selection
                        if (currentSelection) {
                            console.log("There's a selection");
                            const startIndex = Math.max(currentSelection.index - 3, 0); // Start from the first backtick
                            quill.removeFormat(startIndex, 3); // Remove any existing format for the three backticks
                            quill.deleteText(startIndex, 2);
                            quill.formatText(startIndex, 3, 'code-block', true); // Apply the code block format

                            const selectIndex = startIndex + 3; // this literally has to be calculated in separate, otherwise javascript concatenates them as strings lol
                            quill.setSelection(selectIndex, 0); // Set the cursor after the third backtick
                        }
                        event.preventDefault(); // Prevent the third backtick from being typed into the editor
                        consecutiveBackticks = 0; // Reset the counter
                    }

                } else {
                    // Create inline code format
                    if (consecutiveBackticks === 1) {
                        const currentSelection = quill.getSelection();
                        if (currentSelection) {
                            const startIndex = Math.max(currentSelection.index - 1, 0); // Start from the backtick

                            // We delay the application of the format by a few millisecond
                            // to allow the browser to process the initial keystroke. We do this
                            // because if not the browser adds another backtick which is undesirable

                            const formatStartIndex = startIndex + 1; // this literally has to be calculated in separate, otherwise javascript concatenates them as strings lol
                            const formatEndIndex = startIndex + 2; // this literally has to be calculated in separate, otherwise javascript concatenates them as strings lol

                            setTimeout(() => {
                                quill.formatText(formatStartIndex, formatEndIndex, 'code', true);
                            }, 10);

                            // After the format has been applied safely we delete the initial
                            // backtick
                            setTimeout(() => {
                                quill.deleteText(startIndex, formatStartIndex);
                            }, 20);
                        }
                        consecutiveBackticks = 0; // Reset the counter
                    } else {

                        // If any other key is pressed without meeting the conditions above, reset the counter
                        consecutiveBackticks = 0;
                    }
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
