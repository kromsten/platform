import { fromBase64, Msg, MsgDelegate } from "secretjs";
import { getAccount } from "./accounts";
import { Attribute } from "./interfaces";
import { run } from "node-jq";



/* const client = new SecretNetworkClient({
    chainId: "secret-4",
    url: "https://lcd.secret.express",
})
 */


const client = getAccount().secretjs
const querier = client.query

const queryPathToFun = (path : string) : Function => {
    if (path === "/cosmos.staking.v1beta1.Query/Validators") return querier.staking.validators.bind(client)
    else if (path === "/cosmos.staking.v1beta1.Query/Validator") return querier.staking.validator.bind(client)
    throw new Error("Not implemented");
}

const msgMapping : {[type : string] : any} = {
    "/cosmos.staking.v1beta1.MsgDelegate": MsgDelegate
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
    if ("investor" in value) return { options: [ "alice", "bob" ] }
    if ("contract" in value) return { value: "staking"};
    if ("amount" in value) return { options: [ "1000", "2000" ] };
    if ("custom" in value) return { value: decodeCustomValue(value.custom.value_type, value.custom.value)};
    throw new Error("Not implemented");
}


export const constructParams = async (attributes : Attribute[]) : Promise<any> => {

    const params : any = {};

    const groupResults : {[group : number] : any} = {}

    for (const attr of attributes) {

        if (attr.querier && !(attr.querier.group in groupResults)) { 
            
            const res = await queryPathToFun(attr.querier.path)(attr.querier.request);
            groupResults[attr.querier.group] = res;
        }

        if (attr.value) {
            params[attr.key] = decodeValue(attr.value)
        }
    }

    for (const attr of attributes.filter(a => Boolean(a.querier))) {
        const res = groupResults[attr.querier!.group]
        const options : any = await run(attr.querier?.jq_parser!, res, { input: 'json', output: 'json' })
        
        params[attr.key] = {
            options
        }
    }



    return params;

}