import type { Window as KeplrWindow } from "@keplr-wallet/types";
import { initSecretClient, initSecretClientSignable } from "./client";

declare global {
    interface Window extends KeplrWindow {}
}


export const init = async () => {
    
    const connectedBefore = localStorage.getItem('connected')

    if (connectedBefore) {
        await initSecretClientSignable();
    } else {
        await initSecretClient();
    }
}

export default init;