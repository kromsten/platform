import { fromBase64, MsgDelegate, MsgUndelegate, MsgWithdrawDelegationReward, type Msg } from "secretjs";
import { getAccount } from "./accounts";
import { run } from "node-jq";
import type { Attribute } from "../src/interfaces/investments";
import { queryPathToFun } from "$lib/utils";


/* const client = new SecretNetworkClient({
    chainId: "secret-4",
    url: "https://lcd.secret.express",
})
 */


const client = getAccount().secretjs



export const msgMapping : {[type : string] : any} = {
    "/cosmos.staking.v1beta1.MsgDelegate": MsgDelegate,
    "/cosmos.staking.v1beta1.MsgUndelegate": MsgUndelegate,
    "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward": MsgWithdrawDelegationReward
}


const getDefaultParams = (type_url : string) : any => {
    return {};
}

export const getMsg = (type_url : string, params?: any) : Msg | null => {
    if (!(type_url in msgMapping)) return null;
    params ||= getDefaultParams(type_url);
    return new msgMapping[type_url](params)
}



export const decodeCustomValue = (type : string, value : string) : any => {
    const buffer = fromBase64(value); 
    if (type === "String" ) return new TextDecoder().decode(buffer);
    throw new Error("Not implemented");
}


export const decodeValue = (value : any) : any => {
    if ("custom" in value) return { value: decodeCustomValue(value.custom.value_type, value.custom.value)};
    else if ("investor" in value || "amount" in value) return value;
    throw new Error("Not implemented");
}



export enum ToFill  {
    Investor, Amount, CoinAmount
}


export type Params = {
    [key : string] : {
        options? : any
        value?: any
        toFill?: ToFill

    }
}

export const parseAttributes = async (attributes : Attribute[]) : Promise<Params> => {

    const params : Params = {};

    const groupResults : {[group : number] : any} = {}


    for (const attr of attributes) {

        if (!(attr.key in params)) {
            params[attr.key] = {}
        }

        if (attr.querier && !(attr.querier.group in groupResults)) { 
            const res = await queryPathToFun(attr.querier.path, client).bind(client)(attr.querier.request);
            groupResults[attr.querier.group] = res;
        }

        if (attr.value) {

            if ('custom' in attr.value) {
                params[attr.key].value = decodeValue(attr.value)
            } else if ('investor' in attr.value) {
                params[attr.key].toFill = ToFill.Investor
            } else if ('amount' in attr.value) {
                params[attr.key].toFill = ToFill.Amount
            }  else if ('coin_amount' in attr.value) {
                params[attr.key].toFill = ToFill.CoinAmount
            }

        }
    }


    for (const attr of attributes.filter(a => Boolean(a.querier))) {
        const res = groupResults[attr.querier!.group]
        const options : any = await run(attr.querier?.jq_parser!, res, { input: 'json', output: 'json' })
        params[attr.key].options = options
    }


    return params;

}