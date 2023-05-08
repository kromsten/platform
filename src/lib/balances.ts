import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public"
import type { Permit, SecretNetworkClient } from "secretjs"


export const getNativeBalances = async (client: SecretNetworkClient,  address: string) => {
    const response = await client.query.bank.allBalances({ address })
    return response?.balances!
}


export const getContractBalances = async (
    client: SecretNetworkClient, 
    address: string, 
    contract_address: string,
    code_hash?: string,
    permit?: Permit,
    viewing_key?: string
) => {


    let query : any = {
        balance: {
            address
        }
    };

    if (permit) {
        query = {
            with_permit: {
                permit,
                query
            }
        }
    } else if (viewing_key) {
        query.balance.viewing_key = viewing_key
    }


    return await client.query.compute.queryContract({
        contract_address,
        code_hash,
        query
    })
}





export const getViewingKey = async (contractAddress : string) => {
    return await window.keplr?.getSecret20ViewingKey(PUBLIC_SCRT_CHAIN_ID, contractAddress)
} 