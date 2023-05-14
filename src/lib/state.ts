import type { RouterStrategy } from "$interfaces/contracts";
import type { InvestmentAction } from "$interfaces/investments";
import { writable, type Writable } from "svelte/store";
import type { NetworkState } from "../interfaces/networks";
import type { TokenState } from "../interfaces/tokens";



export type Networks = {  [networkId: string] : NetworkState }
export type Tokens = { [tokenId: string] : TokenState } 
export type Strategies = { [tokenId: string] : RouterStrategy[] } 


export const networksState: Writable<Networks> = writable({});
export const tokensState: Writable<Tokens> = writable({});
export const strategyState: Writable<Strategies> = writable({});
export const currentStrategies: Writable<RouterStrategy[]> = writable([]);
export const selectedStrategy: Writable<RouterStrategy> = writable();
export const selectedToken = writable<string | undefined>(undefined);
export const investmentMessages = writable<InvestmentAction[]>([]);
export const walletName = writable<string>(undefined);