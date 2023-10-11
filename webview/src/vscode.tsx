interface RpcRequest {
    funcName: string;
    args: any[];
    id: string;
}

interface RpcResponse {
    id: string;
    result?: any;
    error?: string;
}

export const callVisualStudio = async (funcName: string, ...args: any[]): Promise<any> => {
    return new Promise<any>((resolve, reject) => {
        const id: string = Date.now().toString();  // Unique identifier for this call

        function messageListener(event: MessageEvent) {
            const data: RpcResponse = event.data;
            if (data.id === id) {
                window.removeEventListener('message', messageListener);  // Remove the listener
                if (data.error) {
                    reject(data.error);  // If there's an error, reject the promise
                } else {
                    resolve(data.result);  // Otherwise, resolve with the result
                }
            }
        }

        // Set the event listener before sending the message
        window.addEventListener('message', messageListener);

        // Send the message
        const request: RpcRequest = { funcName, args, id };
        (window as any).vscode.postMessage(request);
    });
}
