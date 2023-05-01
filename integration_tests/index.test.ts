import { readFileSync, readdirSync, writeFileSync } from "fs";
import { sha256 } from "@noble/hashes/sha256";
import { toHex, MsgStoreCode, TxResultCode, MsgInstantiateContractResponse } from "secretjs";
import importedConfig from "./assets/config.json";
import { Account, getAccount } from "./wallets.js";
import { expect } from "bun:test";



type ContractConfig = {
    address?: string;
    codeId?: number;
    codeHash?: string;
}

type Config = {
    contracts: {
        [key: string]: ContractConfig;
    }
}


const ASSET_PATH = "./assets";
const WASM_PATH = `${ASSET_PATH}/wasm`;
const config = importedConfig as Config;


const mainAccount : Account =  getAccount();
const client = mainAccount.secretjs;

const loadContracts = async () => {

    const files = readdirSync(WASM_PATH);


    for (const file of files) {

        const name = file.split(".")[0];
        
        if (!(name in config.contracts)) config.contracts[name] = {};

        if (!config.contracts[name]?.codeId) {
            const wasm = readFileSync(`${WASM_PATH}/${file}`) as Uint8Array;
            const codeHash = toHex(sha256(wasm));
            
            const msg = new MsgStoreCode({
                sender: mainAccount.address,
                wasm_byte_code: wasm,
                source: "",
                builder: "",
            })

            const tx = await client.tx.broadcast([msg], {gasLimit: 5_000_000});

            if (tx.code !== TxResultCode.Success) {
                console.error(tx.rawLog);
            }

            expect(tx.code).toBe(TxResultCode.Success);
            const codeId = Number(tx.arrayLog!.find((x) => x.key === "code_id")!.value);
            
            config.contracts[name].codeId = codeId;
            config.contracts[name].codeHash = codeHash;
        }


        if (!config.contracts[name]?.address) {

            const { codeId, codeHash } = config.contracts[name];

            const tx = await client.tx.compute.instantiateContract(
                {
                  sender: mainAccount.address,
                  code_id: codeId!,
                  code_hash: codeHash!,
                  init_msg: {},
                  label: `${name}-${Date.now()}`,
                },
                { gasLimit: 300_000 }
              );

            expect(tx.code).toBe(TxResultCode.Success);
            const address = MsgInstantiateContractResponse.decode(tx.data[0]).address;
            config.contracts[name].address = address;
        }

        writeFileSync(`${ASSET_PATH}/config.json`, JSON.stringify(config, null, 2));
    }

}


const init = async () => {
    await loadContracts();
}


console.log("Starting integration tests...");
await init();