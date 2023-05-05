type Investor = {
    investor: string;
}

type Contract = {
    contract: string;
}

type Amount = {
    contract: BigInt;
}

type Custom = {
  value_type: string;
  value: string;
}


export type AttributeValue = Investor | Contract | Amount | Custom;


export type ValueQuerier = {
    path: string,
    request: any,
    group: number,
    jq_parser: string,
}


export type Attribute = {
    key: string,
    value?: AttributeValue,
    querier?: ValueQuerier
}



export type InvestParamsResult = {
    name: string;
    attributes: Attribute[];
}
