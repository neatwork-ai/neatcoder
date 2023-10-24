// QuillEditor.tsx

import React, { useEffect, useRef } from 'react';
import ReactQuill, { Quill } from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles
import 'font-awesome/css/font-awesome.min.css';
const Inline = Quill.import('blots/inline');

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
    // ... other formats you might want to use
    "code",  // inline code
    "code-block",  // code block
];

export const QuillEditor: React.FC = () => {
    const [editorContent, setEditorContent] = React.useState('');
    const quillRef = useRef<ReactQuill>(null);

    useEffect(() => {
        if (quillRef.current) {
            const quill = quillRef.current.getEditor();

            let consecutiveBackticks = 0;  // Track consecutive backtick presses

            const handleKeyDown = (event: KeyboardEvent) => {
                if (event.code === "BracketRight") {  // Key code for backtick
                    consecutiveBackticks++;

                    if (consecutiveBackticks === 3) {
                        console.log("Three backticks!");
                        const currentSelection = quill.getSelection(); // Get current selection
                        if (currentSelection) {
                            console.log("There's a selection");
                            const startIndex = Math.max(currentSelection.index - 3, 0); // Start from the first backtick
                            quill.removeFormat(startIndex, 3); // Remove any existing format for the three backticks
                            quill.deleteText(startIndex, 2);
                            quill.formatText(startIndex, 3, 'code-block', true); // Apply the code block format
                            quill.setSelection(startIndex + 3, 0); // Set the cursor after the third backtick
                        }
                        event.preventDefault(); // Prevent the third backtick from being typed into the editor
                        consecutiveBackticks = 0; // Reset the counter
                    }
                } else {
                    if (consecutiveBackticks === 2) {
                        console.log("Two backticks!");
                        const currentSelection = quill.getSelection();
                        if (currentSelection) {
                            console.log("There's a selection");
                            const startIndex = Math.max(currentSelection.index - 2, 0); // Start from the first backtick
                            console.log("startIndex: " + startIndex);
                        //     // // quill.removeFormat(startIndex, 2); // Remove any existing format for the two backticks

                            quill.formatText(0, 1, 'code', true); // THE ERROR IS HERE..
                        //     // delayLoop(1000000000);
                        //     // quill.deleteText(startIndex, 2);
                        //     // delayLoop(1000000000);
                        //     // event.preventDefault();

                        //     // delayLoop(1000000000);

                        //     // quill.deleteText(startIndex, 1);
                        //     // quill.setSelection(startIndex + 2, 0); // Set the cursor after the second backtick
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
    }, []);

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
