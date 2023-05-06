import { expect, test, describe, it} from 'vitest';
import { getAccount } from './accounts';
import configuration from './config';
import type { ContractConfig } from './interfaces';

const account = getAccount();
const client = account.secretjs;

describe('Routes', () => {

    let config : ContractConfig = configuration.contracts["router"];
    expect(config).toBeDefined();
    expect(config.codeId).toBeDefined();
    expect(config.address).toBeDefined();

    const strategy = configuration.contracts["staking_strategy"];

    const contract_address = config.address!;
    const code_hash = config.codeHash!

    test('Contract exists', async () => {
        const res = await client.query.compute.queryContract({
            contract_address,
            code_hash,
            query: { all_strategies: {}}
        })
        expect(res).toBeDefined();
    });


    test("Add route", async () => {
        const res = await client.tx.compute.executeContract({
            sender: account.address,
            contract_address,
            code_hash,
            msg: { add_route: { contract: {
                address: strategy.address!,
                hash: strategy.codeHash!,
            } } }
        }, {
            gasLimit: 85_000,
        })

        console.log("Add route:", res);
    });


});
