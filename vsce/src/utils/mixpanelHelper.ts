// MixpanelHelper.ts
import * as mixpanel from 'mixpanel';
import { CONFIG } from '../config';

class MixpanelHelper {
    private static instance: MixpanelHelper;
    private mixpanelInstance: mixpanel.Mixpanel;

    private constructor(token: string, config?: mixpanel.InitConfig) {
        this.mixpanelInstance = mixpanel.init(token, config);
    }

    static getInstance(config?: mixpanel.InitConfig): MixpanelHelper {
        if (!MixpanelHelper.instance) {
            MixpanelHelper.instance = new MixpanelHelper(CONFIG.mixpanelToken, config); // Use the token from the config file
        }
        return MixpanelHelper.instance;
    }

    trackEvent(eventName: string, properties: mixpanel.PropertyDict = {}): void {
        this.mixpanelInstance.track(eventName, properties, (err) => {
            if (err) {
                console.error('Error sending event to Mixpanel:', err);
            }
        });
    }
}

export default MixpanelHelper;
