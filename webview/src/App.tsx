// import React from 'react';
import './App.css';
import ChatContainer from './ components/chatContainer';

function App() {
  return (
    <div className="App">
      <header>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&display=swap" rel="stylesheet"/>
      </header>
        <h1>Chat with Neatcoder</h1>
        <ChatContainer />
    </div>
  );
}

export default App;
