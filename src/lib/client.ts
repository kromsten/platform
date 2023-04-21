import { PUBLIC_SCRT_CHAIN_ID, PUBLIC_SCRT_ENDPOINT } from "$env/static/public";
import { SecretNetworkClient } from "secretjs";
import { connectWallet } from "./connector";
import { writable } from "svelte/store";

import type { Window as KeplrWindow } from "@keplr-wallet/types";


declare global {
    interface Window extends KeplrWindow {}
}

export const secretClient = writable<SecretNetworkClient>();
export const secretClientSignable = writable<boolean>(false);
export const secretAddress = writable<string>('');



export const initSecretClient = async () => {
    const client = new SecretNetworkClient({
        chainId: PUBLIC_SCRT_CHAIN_ID,
        url: PUBLIC_SCRT_ENDPOINT
    });

    secretClient.set(client);
}


export const initSecretClientSignable = async () => {
    if (window.getOfflineSigner) {
        
        const connected = await connectWallet();
        if (!connected) return;

        const wallet = window.getOfflineSignerOnlyAmino!(PUBLIC_SCRT_CHAIN_ID);
        const account = (await wallet.getAccounts())[0];

        const client = new SecretNetworkClient({
            chainId: PUBLIC_SCRT_CHAIN_ID,
            url: PUBLIC_SCRT_ENDPOINT,
            encryptionUtils: window.getEnigmaUtils!(PUBLIC_SCRT_CHAIN_ID),
            walletAddress: account.address,
            wallet
        });
    
        secretClient.set(client);
        secretClientSignable.set(true);
        secretAddress.set(account.address)
    }
}



// for convinience in non .svelte files
let secretClientValue : SecretNetworkClient | undefined;
let secretClientSignableValue : boolean | undefined;

secretClient.subscribe(value => secretClientValue = value);
secretClientSignable.subscribe(value => secretClientSignableValue = value);


export const getSecretClient = () => secretClientValue;
export const getSecretClientSignable = () => secretClientSignableValue;
