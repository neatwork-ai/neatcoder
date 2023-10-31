import mixpanel, { Dict } from 'mixpanel-browser';
import { CONFIG } from './config';

class MixpanelWebviewHelper {
    private static instance: MixpanelWebviewHelper;
    private userId: string | undefined;

    private constructor() {
        mixpanel.init(CONFIG.mixpanelToken);
        this.initializeUserId();
    }

    private async initializeUserId() {
       this.userId = await this.getUserIdFromExtension();
       mixpanel.identify(this.userId);
    }

    static getInstance(): MixpanelWebviewHelper {
        if (!MixpanelWebviewHelper.instance) {
            MixpanelWebviewHelper.instance = new MixpanelWebviewHelper();
        }
        return MixpanelWebviewHelper.instance;
    }

    private getUserIdFromExtension(): Promise<string> {
        return new Promise((resolve) => {
            const vscode = acquireVsCodeApi();
            vscode.postMessage({ command: 'getUserId' });

            window.addEventListener('message', event => {
                const message = event.data;
                if (message.command === 'userId') {
                    resolve(message.userId);
                }
            });
        });
    }

    trackEvent(eventName: string, properties: Dict = {}): void {
        properties.moduleName = 'Webview';
        mixpanel.track(eventName, properties);
    }
}

export default MixpanelWebviewHelper;
