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
