// QuillEditor.tsx

import React, { useEffect, useRef } from 'react';
import ReactQuill, { Quill } from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles
import 'font-awesome/css/font-awesome.min.css';

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

interface CustomBindingRange {
    index: number;
    length: number;
}

interface CustomBindingContext {
    format: {
        code?: boolean;
    };
}

export const QuillEditor: React.FC = () => {
    const [editorContent, setEditorContent] = React.useState('');
    const quillRef = useRef<ReactQuill>(null);

    useEffect(() => {
        console.log("YOOH")
        if (quillRef.current) {
            console.log("YOOYAHHH")
            const quill = quillRef.current.getEditor();
            const keyboard = quill.getModule('keyboard');

            // Custom key binding for right arrow key
            keyboard.addBinding({
                key: 'right',
                handler: function(range: CustomBindingRange, context: CustomBindingContext) {
                    if (context.format.code) {
                        const CodeBlot = Quill.import('formats/code');
                        // let blot = quill.scroll.descendant(CodeBlot, range.index)[0];
                        const blot = (quill.scroll as any).descendant(CodeBlot, range.index)[0];

                        console.log("blot.length - 1" + (blot.length - 1));
                        console.log("blot.offset(quill.scroll)" + blot.offset(quill.scroll));
                        if (blot && blot.length - 1 === range.index - blot.offset(quill.scroll)) {
                            console.log("bonkers!");
                            // Move the selection out of the code blot
                            quill.setSelection(range.index + 1, 0);

                            return true;
                        } else {
                            console.log("returning false")
                            quill.setSelection(range.index + 1, 0);
                            return false;
                        }
                    } else {
                        quill.setSelection(range.index + 1, 0);
                        return false;
                    }
                }
            });
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
