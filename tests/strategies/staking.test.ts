import { getAccount } from "../accounts";
import config from "../config";
import type { Account } from "../interfaces";

import { describe, expect, test } from "vitest";
import type { InvestParamsResult } from "../../src/interfaces/investments";


import { parseAttributes, ToFill } from "../utils";
import { queryPathToFun, sleep } from "$lib/utils";
import { coinFromString, StakeAuthorizationType, type StakeAuthorization } from "secretjs";


const mainAccount : Account =  getAccount(1);
const client = mainAccount.secretjs;


const authorizeIfNeeded = async (granter : string, grantee : string, validator: string) => {

    const grants = await client.query.authz.grants({ granter, grantee });
    
    const now = Date.now();

    if (grants.grants && grants.grants.length > 0) {
        const grant = grants.grants[0];

        if (grant.expiration && Date.parse(grant.expiration as string) - 5 > now) {
            return;
        }

    }

    const authz =  await client.tx.authz.grant({
        granter: granter,
        grantee: grantee,
        authorization: {
            authorization_type: StakeAuthorizationType.Delegate,
            allow_list: [validator],
            deny_list: [],
            max_tokens: coinFromString("10000000000uscrt")
        } as StakeAuthorization,
        expiration: now + 60 * 60 * 24 * 7
    }) 
    
    console.log("authz", authz)
    expect(authz.code).toBe(0);
}


describe("Staking.wasm", () => {

    const contractConfig = config.contracts["staking_strategy"];

    describe("Investment", async () => {

        const investParams : InvestParamsResult = await client.query.compute.queryContract({
            contract_address: contractConfig.address!,
            code_hash: contractConfig.codeHash!,
            query: { invest_params: {} }
        })

        test('Invest params', async () => {
            
            const [validator, delegator, amount] = investParams.attributes;

            expect(validator.key).toBe("validator_address");
            expect(delegator.key).toBe("delegator_address");
            expect(amount.key).toBe("amount")

            expect(validator.value).toBeNull();
            expect(validator.querier).toBeDefined();
            expect(validator.querier).toBeDefined();
            expect(validator.querier!.path).toContain(("alidators"));
            expect(() => queryPathToFun(validator.querier?.path! + "1", client)).toThrow();
            expect(queryPathToFun(validator.querier?.path!, client)).not.toBeNull();

            expect(delegator.querier).toBeNull();
            expect(delegator.value).toBeDefined();
            expect(delegator.value).toHaveProperty("investor");

            expect(amount.querier).toBeNull();
            expect(amount.value).toBeDefined();
            expect(amount.value).toHaveProperty("amount");

            const params = await parseAttributes(investParams.attributes)

            expect(params).toHaveProperty("validator_address");
            expect(params).toHaveProperty("delegator_address");
            expect(params).toHaveProperty("amount");

            expect(Array.isArray(params.validator_address.options)).toBe(true);
            expect(params.validator_address.options.length).toBeGreaterThan(0);

            expect(params.delegator_address.toFill).toBe(ToFill.Investor);
            expect(params.amount.toFill).toBe(ToFill.Amount);
        })


        test('Invest', async () => {

            const params = await parseAttributes(investParams.attributes)
            
            const amount = "1000000";
            const investor = mainAccount.address;

            const msg : any = {}
            msg[investParams.name] = {};

            for (const [key, param] of Object.entries(params)) {

                let value;

                if (param.value) {
                    value = param.value;
                } else if (param.toFill === ToFill.Amount) {
                    value = amount;
                } else if (param.toFill === ToFill.Investor) {
                    value = investor;
                } else if (param.options) {
                    value = param.options[0];
                } else {
                    throw new Error("Invalid param");
                }

                msg[investParams.name][key] = value;
            }

            await authorizeIfNeeded(
                mainAccount.address, 
                contractConfig.address!, 
                params.validator_address.options[0]!
            )

            const staking = await client.query.staking.delegatorDelegations({delegator_addr: mainAccount.address});
            const delegation = staking.delegation_responses![0]
            let staking_balace_before = delegation ? delegation.balance?.amount ?? "0" : "0";


            const res = await client.tx.compute.executeContract({
                contract_address: contractConfig.address!,
                code_hash: contractConfig.codeHash!,
                sender: mainAccount.address,
                msg
            }, { gasLimit: 60000})


            expect(res.code).toBe(0);

            const staking2 = await client.query.staking.delegatorDelegations({delegator_addr: mainAccount.address});
            const delegation2 = staking2.delegation_responses![0]            
            let staking_balace_after = delegation2 ? delegation2.balance?.amount ?? "0" : "0";
            
            expect(BigInt(staking_balace_after)).toBeGreaterThan(BigInt(staking_balace_before));
            expect(BigInt(staking_balace_after)).toBe(BigInt(staking_balace_before) + BigInt(amount));

        })

    })




    
})
