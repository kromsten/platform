import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import { supportedNetworks, supportedTokens } from "../config";
import type { Window as KeplrWindow } from "@keplr-wallet/types";
import type { Coin } from "secretjs/dist/protobuf/cosmos/base/v1beta1/coin";
import type { NetworkState } from "../interfaces/networks";
import type { LocalStorageState, StorageNetworks, StorageTokens } from "../interfaces/state";
import type { SupportedToken } from "../interfaces/tokens";
import { getContractBalances, getNativeBalances, getViewingKey } from "./balances";
import { initSecretClient, initSecretClientSignable } from "./client";
import { connectWallet } from "./connector";
import { networksState, type Networks, type Tokens } from "./state";
import { getSubscribedValue, toHumanBalance } from "./utils";

declare global {
    interface Window extends KeplrWindow {}
}


export const APP_PREFIX = "platform"


export const defaultState = () => {
    const default_state = localStorage.getItem(`${APP_PREFIX}_state`)
    
    let state : LocalStorageState = {
        networks: {},
        tokens: {}
    }

    if (default_state) {
        state = JSON.parse(default_state)
    } else {

        state.networks =  {
            "secret-4": {
                autoConnect: false
            }
        };

        state.tokens = {
            'uscrt': {
                permit: undefined
            }
        }

        state.networks[PUBLIC_SCRT_CHAIN_ID] = {
            autoConnect: false
        }

        localStorage.setItem(`${APP_PREFIX}_state`, JSON.stringify(state))
    }
    return state
}


const initNetworks = async (storageNetworks : StorageNetworks) => {
    let newNetworks: Networks = {};

    for (const [chainId, network] of Object.entries(storageNetworks)) {
        const networkState : any = {
            connected: false,
        }

        let networkInfo = supportedNetworks[chainId] 

        if (network.autoConnect) {
            networkState.client = await initSecretClientSignable(chainId, networkInfo.nodeUrl)
            if (networkState.client) {
                networkState.address = networkState.client.address;
                networkState.connected = true;
            }
        }

        networkState.client ||= initSecretClient(chainId, networkInfo.nodeUrl)
        newNetworks[chainId] = networkState as NetworkState;
    };
    

    networksState.set(newNetworks)
}





export const initTokens = async (tokens : StorageTokens) => {
    
    const newTokens : Tokens = {};
    let networks : Networks = await getSubscribedValue(networksState);

    const connectedNetworks = Object
        .entries(networks)
        .filter(([_, network]) => network.connected)

    
    const nativeResults = await Promise.allSettled(
        connectedNetworks.map(([_, network]) => getNativeBalances(network.client, network.address))
    ) 


    const nativeBalances = nativeResults
            .filter((result) => result.status === "fulfilled")
            .map((result : any) => result.value as Coin[])



    for (const coins of nativeBalances) {
        for (const coin of coins) {
            const supported = supportedTokens[coin.denom];
            if (supported) {
                newTokens[coin.denom] = {
                    balance: coin.amount,
                    balanceNumber: toHumanBalance(coin.amount, supported.decimals)
                }
            }

        }
    }


    
    const restTokens = Object.entries(tokens).filter((entry) => !newTokens[entry[0]] && entry[0] in supportedTokens);
    networks  = await getSubscribedValue(networksState);
    
    const results = await Promise.allSettled(
        
        restTokens.map(async ([id, storageInfo]) => {
            
            const tokenInfo : SupportedToken = supportedTokens[id]!;
            const networkInfo : NetworkState = networks[tokenInfo.chainId!];

            if (!networkInfo  || tokenInfo.type === "native" || !tokenInfo.address) {
                throw new Error("Runtime error");
            }

            let permit, key;

            if (tokenInfo.chainId == PUBLIC_SCRT_CHAIN_ID) {
                permit = storageInfo.permit;
                if (!permit) {
                    key = await getViewingKey(tokenInfo.address!)
                }

                if (!permit && !key) {
                    throw new Error("Runtime error");
                }
            }

            return await getContractBalances(
                networkInfo.client, 
                networkInfo.address, 
                tokenInfo.address!,
                tokenInfo.hash,
                permit,
                key
            )

        })
    );

    
    const wasmBalances = results.filter((result) => result.status === "fulfilled").map((result : any) => result.value) 


    for (const wasmBalance of wasmBalances) {
        for (const balance of wasmBalance) {
            const supported = supportedTokens[balance.token];
            if (supported) {
                newTokens[balance.token] = {
                    balance: balance.balance,
                    balanceNumber: toHumanBalance(balance.balance, supported.decimals)
                }
            }
        }
    }
}


export const init = async () => {
    
    const state = defaultState()

    const autoConnectChainIds = Object
            .entries(state.networks)
            .filter(([_, network]) => network.autoConnect)
            .map(([chainId, _]) => chainId);

    await connectWallet(autoConnectChainIds)
    .then(() => initNetworks(state.networks))
    .then(() => initTokens(state.tokens))
    
}

export default init;