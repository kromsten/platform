import { contracts } from "$contractConfig"
import type { RouterStrategy, Token } from "$interfaces/contracts";
import type { InvestmentAction } from "$interfaces/investments";
import { supportedTokens } from "../config"
import { queryContract, secretClient } from "./client"


export const getStrategies = async (token: string) : Promise<RouterStrategy[]> => {
    const tokenInfo = supportedTokens[token];
    if (!tokenInfo || !secretClient) return [];


    const query : { token_strategies : { token : Token} } = {
        token_strategies: {
            token: tokenInfo.address 
                ? { snip20: { address: token, hash: tokenInfo.hash! } } 
                : { native: token }
        }
        
    }

    const strategies = await queryContract(
        secretClient,
        query,
        contracts.router.address,
        contracts.router.codeHash,
    );

    return strategies.map((strategy : RouterStrategy) => ({...strategy, contract: strategy.contract ?? contracts.staking_strategy.address}));
}



export const getStrategyMessages = async (strategyAddress: string) : Promise<InvestmentAction[]> => {
    const contractInfo = Object.values(contracts).find((contract) => contract.address === strategyAddress);
    if (!secretClient || !contractInfo) return [];
    const query = { invest_msgs : {} } 
    const actions = await queryContract(
        secretClient,
        query,
        contractInfo.address,
        contractInfo.codeHash,
    );
    return actions;
}