import type { SecretNetworkClient } from "secretjs";
import type { Writable } from "svelte/store";
import { secretClient } from "./client";


export const BALANCE_PRECISION = 3;

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
export const formatAddress = (address: string) => address.slice(0, 6) + "..." + address.slice(-4)




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