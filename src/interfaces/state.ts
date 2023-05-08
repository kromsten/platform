import type { Permit } from "secretjs";

export type StorageNetworks = {
    [networkId: string]: {
        autoConnect: boolean;
    };
}

export type StorageTokens = {
    [tokenId: string]: {
        permit?: Permit;
    };
}

export interface LocalStorageState {
    networks: StorageNetworks
    tokens: StorageTokens
}