import type { Permit } from "secretjs";

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
    viewingKey?: string;
    permit?: Permit
}

