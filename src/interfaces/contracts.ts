
export type Contract = {
    address: string;
    hash: string;
}

export type Native = { native: string };
export type Snip20 = { snip20: Contract };
export type Token = Native | Snip20;


export type RouterStrategy = {
    contract: Contract;
    inputs: Token[];
    outputs: Token[];
}