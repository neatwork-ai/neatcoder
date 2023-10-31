import * as mixpanel from 'mixpanel';
import * as vscode from 'vscode';
import { v4 as uuidv4 } from 'uuid';
import { CONFIG } from '../config';
import * as os from 'os';

class MixpanelHelper {
    private static instance: MixpanelHelper;
    private mixpanelInstance: mixpanel.Mixpanel;
    private userId: string;

    private constructor(token: string, config?: mixpanel.InitConfig) {
        const defaultConfig = {
            geolocate: true
        };

        // If there's any user-provided config, it will overwrite the defaults
        const finalConfig = { ...defaultConfig, ...config };

        this.mixpanelInstance = mixpanel.init(token, finalConfig);

        // Retrieve or generate the userId
        this.userId = this.getUserId();
    }

    static getInstance(config?: mixpanel.InitConfig): MixpanelHelper {
        if (!MixpanelHelper.instance) {
            MixpanelHelper.instance = new MixpanelHelper(CONFIG.mixpanelToken, config);
        }
        return MixpanelHelper.instance;
    }

    private getUserId(): string {
        const userId = vscode.workspace.getConfiguration().get<string>('NeatworkAi.neatcoder.userId');
        if (userId) {
            return userId;
        }

        const newUserId = uuidv4();
        vscode.workspace.getConfiguration().update('NeatworkAi.neatcoder.userId', newUserId, vscode.ConfigurationTarget.Global);
        return newUserId;
    }

   private getExtensionVersion(): string {
        const extension = vscode.extensions.getExtension('NeatworkAi.neatcoder');
        return extension?.packageJSON.version || 'unknown';
    }

    trackEvent(eventName: string, properties: mixpanel.PropertyDict = {}): void {
        properties.distinct_id = this.userId;
        properties.os = os.type();
        properties.extensionVersion = this.getExtensionVersion();

        this.mixpanelInstance.track(eventName, properties, (err) => {
            if (err) {
                console.error('Error sending event to Mixpanel:', err);
            }
        });
    }
}

export default MixpanelHelper;
