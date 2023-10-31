import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import MixpanelWebviewHelper from './mixpanel-webview-helper';
import { Metric } from 'web-vitals';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

function sendToMixpanel(metric: Metric) {
    const mixpanelWebview = MixpanelWebviewHelper.getInstance();

    const eventData = {
        id: metric.id,
        name: metric.name,
        value: metric.value,
        delta: metric.delta,
    };

    mixpanelWebview.trackEvent('WebVitals', eventData);
}

reportWebVitals(sendToMixpanel);
