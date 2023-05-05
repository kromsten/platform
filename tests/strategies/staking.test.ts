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



            const authz =  await client.tx.authz.grant({
                granter: mainAccount.address,
                grantee: contractConfig.address!,
                authorization: {
                    authorization_type: StakeAuthorizationType.Delegate,
                    allow_list: [msg.invest.validator_address],
                    deny_list: [],
                    max_tokens: coinFromString("10000000000uscrt")
                } as StakeAuthorization,
                expiration: Math.round(Date.UTC(2023, 6, 1) / 1000)
            })


            console.log("authz", authz) 
            await sleep(1000);


            let grantee = await client.query.authz.granteeGrants({grantee: contractConfig.address!});
            console.log("grantee", grantee.grants)


            console.log("msg", msg)
            const staking = await client.query.staking.delegatorDelegations({delegator_addr: mainAccount.address});
            console.log("staking", staking.delegation_responses![0])
            
            const res = await client.tx.compute.executeContract({
                contract_address: contractConfig.address!,
                code_hash: contractConfig.codeHash!,
                sender: mainAccount.address,
                msg //: { invest: { amount } }
            }, { gasLimit: 50000})


            console.log("res", res)


            const staking2 = await client.query.staking.delegatorDelegations({delegator_addr: mainAccount.address});
            console.log("staking", staking2.delegation_responses![0])
            
                2 / 0

        })

    })




    
})
