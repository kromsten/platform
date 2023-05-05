import { PUBLIC_SCRT_CHAIN_ID, PUBLIC_SCRT_ENDPOINT } from "$env/static/public";
import { SecretNetworkClient } from "secretjs";
import { connectWallet, secretAddress } from "./connector";
import { writable } from "svelte/store";

import type { Window as KeplrWindow } from "@keplr-wallet/types";


declare global {
    interface Window extends KeplrWindow {}
}


export let secretClient : SecretNetworkClient | undefined = undefined;
export const secretClientStore = writable<SecretNetworkClient>();
export const secretClientSignableStore = writable<boolean>(false);

secretClientStore.subscribe(value => secretClient = value)


export const initSecretClient = async () => {
    const client = new SecretNetworkClient({
        chainId: PUBLIC_SCRT_CHAIN_ID,
        url: PUBLIC_SCRT_ENDPOINT
    });
    secretClientStore.set(client);
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
    
        secretClientStore.set(client);
        secretClientSignableStore.set(true);
        secretAddress.set(account.address)
    }
}


// for convinience in non .svelte files
let secretClientValue : SecretNetworkClient | undefined;
let secretClientSignableValue : boolean | undefined;

secretClientStore.subscribe(value => secretClientValue = value);
secretClientSignableStore.subscribe(value => secretClientSignableValue = value);


export const getSecretClient = () => secretClientValue;
export const getSecretClientSignable = () => secretClientSignableValue;
