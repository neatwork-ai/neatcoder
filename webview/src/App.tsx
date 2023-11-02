// import React from 'react';
import './App.css';
import ChatContainer from './ components/chatContainer';
import { useEffect } from 'react';
import MixpanelWebviewHelper from './mixpanel-webview-helper';
// import 'highlight.js/styles/atom-one-dark.css';

function App() {
    useEffect(() => {
        try {
            const mixpanelWebview = MixpanelWebviewHelper.getInstance();
            console.log("MixpanelWebviewHelper initialized", mixpanelWebview);
        } catch (error) {
            console.error("Error initializing MixpanelWebviewHelper:", error);
        }
    }, []);
  return (
    <div className="App">
      <header>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&display=swap" rel="stylesheet"/>
      </header>
        <ChatContainer />
    </div>
  );
}

export default App;
