import { toHex, MsgStoreCode, TxResultCode, MsgInstantiateContractResponse } from "secretjs";
import { readFileSync, readdirSync, writeFileSync } from "fs";
import { sha256 } from "@noble/hashes/sha256";
import { getAccount } from "./accounts";
import { expect, test, describe, it} from 'vitest';
import type { Account, ContractConfig} from "./interfaces";


import config from "./config";

const ASSET_PATH = "assets";
const WASM_PATH = `${ASSET_PATH}/wasm`;


const mainAccount : Account =  getAccount();
const client = mainAccount.secretjs;


const loadContracts = async () => {


    const files = readdirSync(WASM_PATH);

    const existing = Object.keys(config.contracts).filter((k) => config.contracts[k].codeId && config.contracts[k].address);

    for (const file of files) {

        const name = file.split(".")[0];

        if (existing.includes(name)) continue;
        
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

        expect(config.contracts[name].codeId).toBeDefined();

        writeFileSync(`${ASSET_PATH}/config.json`, JSON.stringify(config, null, 2));    

        if (!config.contracts[name]?.address) {

            const { codeId, codeHash } = config.contracts[name];

            const tx = await client.tx.compute.instantiateContract(
                {
                  sender: mainAccount.address,
                  code_id: codeId!,
                  code_hash: codeHash!,
                  init_msg: { 
                    default_validator: "secretvaloper1ap26qrlp8mcq2pg6r47w43l0y8zkqm8aynpdzc"
                  },
                  label: `${name}-${Date.now()}`,
                },
                { gasLimit: 300_000 }
            );


            expect(tx.code).toBe(TxResultCode.Success);
            const address = MsgInstantiateContractResponse.decode(tx.data[0]).address;
            config.contracts[name].address = address;
        }


        expect(config.contracts[name].address).toBeDefined();

        writeFileSync(`${ASSET_PATH}/config.json`, JSON.stringify(config, null, 2));
    }

}



describe('Init', () => {
    test('All contract are deployed and instantiated', async () => {
        await loadContracts();
        expect(Object.keys(config.contracts).length)
            .toBeGreaterThanOrEqual(readdirSync(WASM_PATH).length);
    });
});


if (Object.keys(config.contracts).length) {
    describe.each(Object.entries(config.contracts)) 
        ('Contract configuration', (name : string, config : ContractConfig) => {
            it(name + ".wasm have codeId and address", () => {
                expect(config.codeId).toBeDefined();
                expect(config.address).toBeDefined();
            })
        }
    );
}

