import { SecretNetworkClient } from "secretjs";
import { connectWallet } from "./connector";

import type { Window as KeplrWindow } from "@keplr-wallet/types";


declare global {
    interface Window extends KeplrWindow {}
}


export let secretClient : SecretNetworkClient | undefined = undefined;


export const initSecretClient = (chainId: string, url: string) => {
    const client = new SecretNetworkClient({chainId, url});
    return client;
}



export const initSecretClientSignable = async (chainId: string, url: string) => {

    if (window.getOfflineSigner) {
        
        const connected = await connectWallet(chainId);
        if (!connected) return;

        const wallet = window.getOfflineSignerOnlyAmino!(chainId);
        const account = (await wallet.getAccounts())[0];

        const client = new SecretNetworkClient({
            chainId,
            url,
            encryptionUtils: window.getEnigmaUtils!(chainId),
            walletAddress: account.address,
            wallet
        });
        
        return client;
    }
}
