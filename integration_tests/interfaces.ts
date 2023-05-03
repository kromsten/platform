import { type SecretNetworkClient, type Wallet } from "secretjs";
import { QueryClient} from "@cosmjs/stargate";

export type ContractConfig = {
    address?: string;
    codeId?: number;
    codeHash?: string;
}

export  type Config = {
    contracts: {
        [key: string]: ContractConfig;
    }
}

export type Account = {
    address: string;
    mnemonic: string;
    wallet: Wallet;
    secretjs: SecretNetworkClient;
    queryClient?: QueryClient;
};



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


type AttributeValue = Investor | Contract | Amount | Custom;


type ValueQuerier = {
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


/* export class StargateClientQueryClient extends StargateClient {

    static override async connect(endpoint: string | HttpEndpoint, options?: StargateClientOptions): Promise<StargateClientQueryClient> {
        const client =  await StargateClient.connect(endpoint, options);
        return client as StargateClientQueryClient;
    }


    client() {
        return this.getQueryClient();
    }
}     */