import { SecretNetworkClient } from "secretjs";
import { connectWallet } from "./connector";

import type { Window as KeplrWindow } from "@keplr-wallet/types";
import { networksState } from "./state";
import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";


declare global {
    interface Window extends KeplrWindow {}
}


export let secretClient : SecretNetworkClient | undefined = undefined;


networksState.subscribe(networks => {
    if (PUBLIC_SCRT_CHAIN_ID in networks) {
        secretClient = networks[PUBLIC_SCRT_CHAIN_ID].client;
    }
})



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



export const queryContract = async (
    client : SecretNetworkClient, 
    query: any,
    contract_address : string,
    code_hash?: string,
) : Promise<any> => {
    return await client.query.compute.queryContract({
        contract_address,
        code_hash,
        query
    })
}

export const executeContract = async (
    client : SecretNetworkClient, 
    msg: any,
    contract_address : string,
    code_hash?: string,
    sent_funds?: any[],
) => {
    return await client.tx.compute.executeContract({
        contract_address,
        sender: client.address,
        code_hash,
        msg,
        sent_funds,
    })
}
