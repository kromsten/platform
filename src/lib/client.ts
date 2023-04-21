import { PUBLIC_SCRT_CHAIN_ID, PUBLIC_SCRT_ENDPOINT } from "$env/static/public";
import { SecretNetworkClient } from "secretjs";
import { writable } from "svelte/store";
import type { Window as KeplrWindow } from "@keplr-wallet/types";
import { WalletType } from "../interfaces/wallets";
import type { WalletConnector } from "../interfaces/functions";


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




export const detectWallet = async () : Promise<WalletType | undefined>  => {
    return WalletType.Keplr
}



export const connectWallet : WalletConnector = async (wallet? : WalletType) => {
    
    wallet ??= await detectWallet()

    let connected = false;

    if (wallet === WalletType.Keplr) {
        connected =  await connectKeplr();
    }

    if (!connected) {
        localStorage.removeItem("connected");
    }

    return connected;
}


export const connectKeplr : WalletConnector = async () => {

    try {
        await window.keplr!.enable(PUBLIC_SCRT_CHAIN_ID);
        return true;
      } catch (e : any) {
        console.error(e.message)
        return false;
      }
}