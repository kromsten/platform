import { PUBLIC_SCRT_CHAIN_ID, PUBLIC_SCRT_ENDPOINT, PUBLIC_SSCRT_ADDRESS, PUBLIC_SSCRT_CODE_HASH } from "$env/static/public";
import type { SupportedNetworks } from "../interfaces/networks";
import type { SupportedToken } from "../interfaces/tokens";


export type TokensOfInterest = { [id: string] : SupportedToken }

export const supportedTokens : TokensOfInterest = {
    uscrt: {
        chainId: PUBLIC_SCRT_CHAIN_ID,
        decimals: 6,
        type: "native",
        symbol: "SCRT",
        
    },

    /* secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek: {
        chainId: "secret-4",
        decimals: 6,
        type: "cw20",
        symbol: "sSCRT",
        address: "secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek",
        hash: "AF74387E276BE8874F07BEC3A87023EE49B0E7EBE08178C49D0A49C3C98ED60E"
    } */
}



export const supportedNetworks : SupportedNetworks = {
    "secret-4": { nodeUrl: "https://lcd.mainnet.secretsaturn.net:443" }
};


supportedNetworks[PUBLIC_SCRT_CHAIN_ID] = {
    nodeUrl: PUBLIC_SCRT_ENDPOINT,
};

supportedTokens[PUBLIC_SSCRT_ADDRESS] = {
    chainId: PUBLIC_SCRT_CHAIN_ID,
    decimals: 6,
    type: "cw20",
    symbol: "sSCRT",
    address: PUBLIC_SSCRT_ADDRESS,
    hash: PUBLIC_SSCRT_CODE_HASH
}


export const defaultTokens = ['uscrt', PUBLIC_SSCRT_ADDRESS]