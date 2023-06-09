import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import { defaultTokens, supportedNetworks, supportedTokens } from "../config";
import type { Window as KeplrWindow } from "@keplr-wallet/types";
import type { Coin } from "secretjs/dist/protobuf/cosmos/base/v1beta1/coin";
import type { NetworkState } from "../interfaces/networks";
import type { LocalStorageState, StorageNetworks, StorageTokens } from "../interfaces/state";
import type { SupportedToken } from "../interfaces/tokens";
import { getContractBalances, getNativeBalances, getViewingKey } from "./balances";
import { initSecretClient, initSecretClientSignable } from "./client";
import { connectWallet } from "./connector";
import { currentStrategies, investmentMessages, networksState, selectedStrategy, selectedToken, strategyState, tokensState, type Networks, type Strategies, type Tokens } from "./state";
import { getSubscribedValue, toHumanBalance } from "./utils";
import { getStrategies, getStrategyMessages } from "./strategies";
import type { RouterStrategy } from "$interfaces/contracts";
import { mockStrategies, mockStrategyMessages } from "../config/demo";

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

        state.tokens = Object.fromEntries(defaultTokens.map((token : string) => ([ token, { permit: undefined }])));

        state.networks[PUBLIC_SCRT_CHAIN_ID] = {
            autoConnect: false
        }

        localStorage.setItem(`${APP_PREFIX}_state`, JSON.stringify(state))
    }
    return state
}


const initNetworks = async () => {
    const storageNetworks = defaultState().networks;
    let newNetworks: Networks = {};

    for (const [chainId, network] of Object.entries(storageNetworks)) {
        const networkState : any = {
            connected: false,
        }

        let networkInfo = supportedNetworks[chainId] 

        if (networkInfo && network.autoConnect) {
            networkState.client = await initSecretClientSignable(chainId, networkInfo.nodeUrl)
            if (networkState.client) {
                networkState.address = networkState.client.address;
                networkState.connected = true;
            }

            networkState.client ||= initSecretClient(chainId, networkInfo.nodeUrl)
            newNetworks[chainId] = networkState as NetworkState;
        }

    };
    

    networksState.set(newNetworks)
}





export const initTokens = async () => {
    
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
                    balanceNumber: toHumanBalance(coin.amount, supported.decimals),
                }
            }
        }
    }


    const defTokens = defaultState().tokens;
    const restTokens = Object.entries(defTokens).filter((entry) => !newTokens[entry[0]] && entry[0] in supportedTokens);
    
    networks  = await getSubscribedValue(networksState);
    
    const results = await Promise.allSettled(
        
        restTokens.map(async ([id, storageInfo]) => {

            
            const tokenInfo : SupportedToken = supportedTokens[id]!;
            const networkInfo : NetworkState = networks[tokenInfo.chainId!];

            if (!networkInfo  || tokenInfo.type === "native" || !tokenInfo.address) {
                throw new Error("Runtime error");
            }

            let permit, key;

            if (tokenInfo.address) {
                permit = storageInfo.permit
                key = await getViewingKey(tokenInfo.address)
            }



            return {
                balance: await getContractBalances(
                    networkInfo.client, 
                    networkInfo.address, 
                    tokenInfo.address!,
                    tokenInfo.hash,
                    permit,
                    key
                ) || "0",
                token: tokenInfo.address!,
                permit,
                key
            }

        })
    );

    
    const wasmProcessesed = results.filter((result) => result.status === "fulfilled").map((result : any) => result.value) 

    for (const processed of wasmProcessesed) {
        const supported = supportedTokens[processed.token];
        if (supported) {
            newTokens[processed.token] = {
                balance: processed.balance,
                balanceNumber: toHumanBalance(processed.balance, supported.decimals),
                permit: processed.permit,
                viewingKey: processed.key
            }
        }
        
    }

    tokensState.set(newTokens)
}


export const init = async () => {
    
    const state = defaultState()

    const autoConnectChainIds = Object
            .entries(state.networks)
            .filter(([_, network]) => network.autoConnect)
            .map(([chainId, _]) => chainId);

    await connectWallet(autoConnectChainIds)
    .then(() => initNetworks())
    .then(() => initTokens())


    selectedToken.subscribe(async (token) => {
        if (!token) return;
        const strategies : Strategies = await getSubscribedValue(strategyState);
        let tokenStrategies : RouterStrategy[] = [];
        if (token in strategies) {
            tokenStrategies = strategies[token];
        } else {
            tokenStrategies = await getStrategies(token);
        }

        const mock = mockStrategies[token] ?? []; 
        tokenStrategies = [...tokenStrategies, ...mock] 

        currentStrategies.set(tokenStrategies)
    })

    selectedStrategy.subscribe((strategy) => {
        if (!strategy) return;

        if (strategy.demo) {
            investmentMessages.set(mockStrategyMessages[strategy.contract.address])

        } else {
            getStrategyMessages(strategy.contract.address)
            .then((messages) => investmentMessages.set(messages))
        }

    })


    
}

export default init;