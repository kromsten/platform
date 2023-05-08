import { PUBLIC_SCRT_CHAIN_ID, PUBLIC_SCRT_ENDPOINT } from "$env/static/public";
import type { SupportedNetworks } from "../interfaces/networks";
import type { SupportedToken } from "../interfaces/tokens";


export type TokensOfInterest = { [id: string] : SupportedToken }

export const supportedTokens : TokensOfInterest = {
    uscrt: {
        chainId: PUBLIC_SCRT_CHAIN_ID,
        decimals: 6,
        type: "native",
        symbol: "SCRT",
    }
}



export const supportedNetworks : SupportedNetworks = {
    "secret-4": { nodeUrl: "https://lcd.mainnet.secretsaturn.net:443" }
};


supportedNetworks[PUBLIC_SCRT_CHAIN_ID] = {
    nodeUrl: PUBLIC_SCRT_ENDPOINT,
};