import { QueryClient, StargateClient } from "@cosmjs/stargate";
import { SecretNetworkClient, Wallet } from "secretjs";
import { Account } from "./interfaces";
import { Tendermint37Client } from "@cosmjs/tendermint-rpc";


const a = "grant rice replace explain federal release fix clever romance raise often wild taxi quarter soccer fiber love must tape steak together observe swap guitar";
const b = "jelly shadow frog dirt dragon use armed praise universe win jungle close inmate rain oil canvas beauty pioneer chef soccer icon dizzy thunder meadow";
const c = "chair love bleak wonder skirt permit say assist aunt credit roast size obtain minute throw sand usual age smart exact enough room shadow charge";

const mnemonics = [a,/*  b, c */];

const accounts: Account[] = new Array<Account>(mnemonics.length);

for (let i = 0; i < mnemonics.length; i++) {
    const mnemonic = mnemonics[i];
    const wallet = new Wallet(mnemonic);
    const account : Account = {
        address: wallet.address,
        mnemonic: mnemonic,
        wallet,
        secretjs: new SecretNetworkClient({
            url: "http://localhost:1317",
            wallet: wallet,
            walletAddress: wallet.address,
            chainId: "secretdev-1",
        }),
    };
    
    account.secretjs.tx.distribution.withdrawDelegatorReward
    Tendermint37Client.connect("http://localhost:26657")
    //StargateClient.connect("http://localhost:26657")
    .then((client) => account.queryClient = new QueryClient(client));
  
    accounts[i] = account;

}

export const getAccount = (index : 0 | 1 | 2 = 0) => {
    return accounts[index];
}
