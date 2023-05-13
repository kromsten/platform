import type { TokenState } from "$interfaces/tokens";
import type { SecretNetworkClient } from "secretjs";
import type { Writable } from "svelte/store";
import { supportedTokens } from "../config";
import { secretClient } from "./client";


export const BALANCE_PRECISION = 2;

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

export const formatAddress = (address: string, symbols: number = 4) => 
    address.slice(0, symbols+2) + "..." + address.slice(-symbols)


export const formatBalance = (denom: string, state: TokenState) => {
    const info = supportedTokens[denom];
    const human = state.balanceNumber ?? toHumanBalance(state.balance, info.decimals);
    return [info.name ?? info.symbol, Number.isInteger(human) ? human.toFixed(0) : human.toFixed(BALANCE_PRECISION)]
}


export const queryPathToFun = (path : string, client? : SecretNetworkClient) : Function => {
    client ||= secretClient; 
    if (path === "/cosmos.staking.v1beta1.Query/Validators") return client!.query.staking.validators.bind(client);
    throw new Error("Not implemented");
}



export const toHumanBalance = (balance: string, decimals: number) => {
    return Number(BigInt(balance) * BigInt(10**BALANCE_PRECISION) / BigInt(10 ** decimals)) / (10**BALANCE_PRECISION);
}

export const getSubscribedValue = async (store : Writable<any> ) : Promise<any> => {
    return new Promise((resolve, _) => resolve(store.subscribe(value => resolve(value))))
}