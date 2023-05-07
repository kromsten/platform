import type { SecretNetworkClient,Wallet } from "secretjs";

export type ContractConfig = {
    address?: string;
    codeId?: number;
    codeHash?: string;
}

export  type Config = {
    contracts: {
        [key: string]: ContractConfig;
    }
}

export type Account = {
    address: string;
    mnemonic: string;
    wallet: Wallet;
    secretjs: SecretNetworkClient;
};


export type Contract = {
    address: string;
    hash: number;
}

export type Native = { native: string };
export type Snip20 = { snip20: Contract };

export type Token = Native | Snip20;

export type Strategy = {
    inputs: Token[];
    outputs: Token[];
}