// QuillEditor.tsx

import React, { useEffect, useRef } from 'react';
import ReactQuill, { Quill } from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles
import 'font-awesome/css/font-awesome.min.css';
import deltaToMarkdown from '../quillToMarkdown/fromDelta';
import { RangeStatic } from 'quill';

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
            let consecutiveBackticks = 0;

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

                console.log('Command + Shift + I pressed');
                quill.format('code', true);

                if (consecutivePresses === 2) {
                    quill.format('code-block', true);
                }
            });

            console.log("keyboard.bindings: " + JSON.stringify(keyboard.bindings));

            const handleKeyDown = (event: KeyboardEvent) => {
                if (event.key === 'Enter' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
                    handleSend();
                }

                if (event.key === 'Enter') {
                    const selection = quill.getSelection();
                    const format = quill.getFormat(selection!);

                    if (format.code) {
                        // If inside inline code, prevent the newline
                        event.preventDefault();
                        return;
                    }
                }

                if (event.key !== 'I' && event.key !== 'Meta' && !event.shiftKey && !event.ctrlKey && !event.altKey) {
                    // Reset presses
                    console.log("YIKEE: "+ event.key);
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
