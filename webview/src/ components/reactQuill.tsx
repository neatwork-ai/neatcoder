// QuillEditor.tsx

import React from 'react';
import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';  // import styles

const CustomToolbar: React.FC = () => {
    return (
        <div id="toolbar">
            {/* Add other toolbar items as needed */}
            <button className="ql-code">Code</button>
            <button className="ql-code-block">Code Block</button>
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

    return (
        <div>
            <CustomToolbar />
            <div className='ql-container-decorator'>
                <ReactQuill
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
