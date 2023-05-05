import type { SecretNetworkClient } from "secretjs";
import { secretClient } from "./client";



export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
export const formatAddress = (address: string) => address.slice(0, 6) + "..." + address.slice(-4)




export const queryPathToFun = (path : string, client? : SecretNetworkClient) : Function => {
    client ||= secretClient; 
    if (path === "/cosmos.staking.v1beta1.Query/Validators") return client!.query.staking.validators.bind(client);
    throw new Error("Not implemented");
}