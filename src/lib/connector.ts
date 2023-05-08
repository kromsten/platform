import type { WalletConnector } from "../interfaces/functions";
import { WalletType } from "../interfaces/enums";
import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import { networksState } from "./state";
import { initSecretClientSignable } from "./client";
import { supportedNetworks } from "../config";



export const detectWallet = async () : Promise<WalletType | undefined>  => {
    return WalletType.Keplr
}


export const connectWallet : WalletConnector = async (chainId: string | string[], wallet? : WalletType) => {
    if (!chainId.length) return false;

    wallet ??= await detectWallet()
    let connected = false;
    if (wallet === WalletType.Keplr) { connected =  await connectKeplr(chainId); }
    return connected;
}



export const connectKeplr : WalletConnector = async (chainId : string | string[]) => {
    try {
        await window.keplr!.enable(chainId);
        return true;
      } catch (e : any) {
        console.error(e.message)
        return false;
      }
}



export const connectChain = async (chainId : string) => {
  const supported = supportedNetworks[chainId];
  if (!supported) return;
  
  return connectWallet(chainId)
  .then(async ok => {

    if (ok) {
      const client = await initSecretClientSignable(chainId, supported.nodeUrl)

      if (client) {

        networksState.update((networks) => {
          return {
            ...networks,
            [chainId]: {
              connected: true,
              client: client,
              address: client.address
            }
          }
        })
      }
    }
  })
} 


export const connectSecret = async () => {
  return await connectChain(PUBLIC_SCRT_CHAIN_ID)
} 





export const disconnectWallet = async (chainId : string) => {}