// import React, { useEffect, useRef } from 'react';
// import Quill from 'quill';
// import SVGButton from './sendButton';

// // Define Custom Inline Blot for Inline Code, as previously discussed
// let Inline = Quill.import('blots/inline');
// class InlineCodeBlot extends Inline { }
// InlineCodeBlot.blotName = 'inlineCode';
// InlineCodeBlot.className = 'one-liner-code';
// InlineCodeBlot.tagName = 'span';

// Quill.register(InlineCodeBlot);

// const QuillEditor: React.FC<{ onContentChange: (content: any) => void }> = ({ onContentChange }) => {
//   const editorRef = useRef<HTMLDivElement | null>(null);
//   const quillInstance = useRef<Quill | null>(null);

//   useEffect(() => {
//     if (editorRef.current) {
//         const toolbarOptions = [
//             ['bold', 'italic', 'underline'],  // Sample existing buttons
//             ['inlineCode']  // Our custom button
//         ];

//         quillInstance.current = new Quill(editorRef.current, {
//             modules: {
//                 toolbar: toolbarOptions,  // Use the toolbar options here
//             },
//             placeholder: 'Start typing...',
//         });

//         if (quillInstance.current) {
//             quillInstance.current.on('text-change', function(delta, oldDelta, source) {
//                 if (source === 'user') {
//                     // Handle the text change event if needed
//                 }
//             });

//             // Bind the button to the custom blot
//             quillInstance.current.getModule('toolbar').addHandler('inlineCode', function() {
//                 let range = quillInstance.current?.getSelection();
//                 if (range) {
//                     quillInstance.current?.format('inlineCode', true);
//                 }
//             });
//         }
//     }
// },
// []);


//   return (
//     <div className="textBox">
//       <div>
//       <div ref={editorRef} data-placeholder="Type your message"></div>
//       <SVGButton onClick={() => {}} />
//       </div>
//     </div>
//   );
// }

// export default QuillEditor;
