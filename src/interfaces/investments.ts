import type { Token } from "./contracts";

type Investor = {
    investor: string;
}

type Contract = {
    contract: string;
}

type Amount = {
    contract: BigInt;
}

type CoinAmount = {
    amount: BigInt;
}

type ViewingKey = {
    viewing_key: string;
}

type StringValue = {
    string_value: string;
}

type NumberValue = {
    number_value: number;
}
 

type Custom = {
  value_type: string;
  value: string;
}


export type AttributeValue = Investor | Contract | Amount | CoinAmount | ViewingKey | StringValue | NumberValue | Custom;




export type ValueQuerier = {
    path: string,
    request: any,
    group: number,
    jq_parser: string,
}


export interface Attribute {
    key: string;
    optional: boolean;
    value?: AttributeValue
    querier?: ValueQuerier;
}


export type InvestParamsResult = {
    name: string;
    attributes: Attribute[];
}



interface StringAttributeValue {
    value: string;
}

interface NumberAttributeValue {
    value: number;
}

interface CustomAttributeValue {
    value_type: string;
    value: ArrayBuffer;
}



export interface QueryBuilder {
    key: string;
    value?: AttributeValue
    querier?: ValueQuerier;
}



export interface Reward {
    token: Token;
    amount: number;
}

export type IssuedToken = Reward;


export enum ActionClass {
    CreateViewingKey,
    SwapToken,
    Allowance,
    Mint,
    Transfer,
    Staking,
    Claiming,
    Unknown,
}

export  enum ActionRequirementType {
    ViewingPermission,
    Allowance,
    Authz,
    AdminRight,
    Whitelist,
}


export interface MessageBuilder {
    name: String,
    attributes: Attribute[]
}




export  interface InvestmentAction {
    chain_id: string;
    type_url: string;
    attributes: Attribute[];
    exposes_investor: boolean;
    issued_tokens?: IssuedToken[];
    optional: boolean;
    description?: string;
    class: ActionClass;
    action_requirements?: ActionRequirementType[];
    independent_action_requirements?: ActionRequirementType[];
    unbonding?: Date;
}



