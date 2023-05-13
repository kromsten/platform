import type { WalletConnector } from "../interfaces/functions";
import { WalletType } from "../interfaces/enums";
import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import { networksState, walletName } from "./state";
import { initSecretClientSignable } from "./client";
import { supportedNetworks } from "../config";
import { APP_PREFIX, initTokens } from "$lib";
import type { LocalStorageState } from "$interfaces/state";
import { browser } from "$app/environment";
import { secretDevInfo } from "./misc";
import { getSubscribedValue } from "./utils";



export const detectWallet = async () : Promise<WalletType | undefined>  => {
    return WalletType.Keplr
}


export const connectWallet : WalletConnector = async (chainId: string | string[], wallet? : WalletType) => {
    if (!chainId.length) return false;

    wallet ??= await detectWallet()
    let connected = false;
    if (wallet === WalletType.Keplr) { connected =  await connectKeplr(chainId); }

    if (!(await getSubscribedValue(walletName))) {
      const keyInfo = await window.keplr?.getKey(Array.isArray(chainId) ? chainId[0] : chainId);

      if (keyInfo?.name) {
        walletName.set(keyInfo.name)
      }
    }

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

  if (PUBLIC_SCRT_CHAIN_ID === 'secretdev-1') {
    await window.keplr?.experimentalSuggestChain(secretDevInfo)
  }

  await connectChain(PUBLIC_SCRT_CHAIN_ID).then(initTokens)

} 







export const disconnectWallet = async (chainId : string) => {}






networksState.subscribe(networks => {
  if (!browser || Object.keys(networks).length == 0) return;

  const readState = localStorage.getItem(`${APP_PREFIX}_state`)
  if (!readState) return;
  const state : LocalStorageState = JSON.parse(readState)
  Object.entries(networks)
    .forEach(([chainId, network]) => {
      state.networks[chainId] = {autoConnect: network.connected}
  })
  localStorage.setItem(`${APP_PREFIX}_state`, JSON.stringify(state))
})