// import React from 'react';
import './App.css';
import ChatContainer from './chatContainer';

function App() {
  return (
    <div className="App">
      <header>
      <meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src vscode-resource: https:; script-src vscode-resource:; style-src vscode-resource:;" />
      </header>
      <h1>Chat with OpenAI</h1>
      <ChatContainer />
    </div>
  );
}

export default App;
