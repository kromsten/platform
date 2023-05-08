import { writable, type Writable } from "svelte/store";
import type { NetworkState } from "../interfaces/networks";
import type { TokenState } from "../interfaces/tokens";



export type Networks = {  [networkId: string] : NetworkState }
export type Tokens = { [tokenId: string] : TokenState } 

export const networksState: Writable<Networks> = writable({});
export const tokensState: Writable<Tokens> = writable({});
