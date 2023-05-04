import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import type { WalletConnector } from "../interfaces/functions";
import { WalletType } from "../interfaces/enums";
import { writable } from "svelte/store";
import { secretClientSignable } from "./client";


export const secretAddress = writable<string>('');
export const connectedWallet = writable<WalletType | null>();


export const detectWallet = async () : Promise<WalletType | undefined>  => {
    return WalletType.Keplr
}


export const connectWallet : WalletConnector = async (wallet? : WalletType) => {
    wallet ??= await detectWallet()

    let connected = false;

    if (wallet === WalletType.Keplr) { connected =  await connectKeplr(); }

    if (!connected) { localStorage.removeItem("connected"); } 
    else { localStorage.setItem("connected", "true"); }

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


export const disconnectWallet = async () => {
    localStorage.removeItem("connected");
    secretAddress.set("");
    connectedWallet.set(null);
    secretClientSignable.set(false);
}