import type { SecretNetworkClient } from "secretjs";
import type { WalletType } from "./enums";

export type SupportedNetwork = {
    nodeUrl: string;
    ibcChannel?: string
}

export type SupportedNetworks = { [id: string]: SupportedNetwork }


export type NetworkState = {
    connected: boolean;
    address : string;
    client: SecretNetworkClient;
}