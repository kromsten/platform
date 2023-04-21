import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import type { WalletConnector } from "../interfaces/functions";
import { WalletType } from "../interfaces/enums";

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
    } else {
        localStorage.setItem("connected", "true");
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
