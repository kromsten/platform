import type { Writable } from "svelte/store";
import type { SecretNetworkClient } from "secretjs";
import type Scrt from "$components/logos/scrt.svelte";

export type SupportedToken = {
    type: "native" | "cw20";
    decimals: number;
    symbol: string;
    chainId: string;
    address?: string;
    hash?: string;
    name?: string;

}


export interface TokenState {
    balance: string;
    balanceNumber: number;
}

