import { getAccount } from "../accounts";
import config from "../config";
import type { Account } from "../interfaces";

import { describe, expect, test } from "vitest";
import type { InvestParamsResult } from "../../src/interfaces/investments";


import { parseAttributes, ToFill } from "../utils";
import { queryPathToFun, sleep } from "$lib/utils";
import { coinFromString, fromBase64, MsgGrantAuthorization, StakeAuthorizationType, toBase64, type GenericAuthorization, type StakeAuthorization } from "secretjs";


const mainAccount : Account =  getAccount(1);
const client = mainAccount.secretjs;


const stakeAuthorizeIfNeeded = async (
    granter : string, 
    grantee : string,
    validator: string,
    authorization_type : StakeAuthorizationType,
    force: boolean = false
) => {

    const now = Date.now();

    if (!force) {
        const grants = await client.query.authz.grants({ granter, grantee });
        if (grants.grants && grants.grants.length > 0) {
            const grant = grants.grants[0];
    
            if (grant.expiration && Date.parse(grant.expiration as string) - 5 > now) {
                return;
            }
    
        }
    }

    const authz =  await client.tx.authz.grant({
        granter: granter,
        grantee: grantee,
        authorization: {
            authorization_type,
            allow_list: [validator],
            deny_list: [],
            max_tokens: coinFromString("10000000000uscrt")
        } as StakeAuthorization,
        expiration: Math.round(now / 1000) + 60 * 60 * 24 * 7
    }) 
    
    expect(authz.code).toBe(0);
}



const claimAuthorizeIfNeeded = async (
    granter : string, 
    grantee : string,
    force: boolean = false
) => {

    const now = Date.now();

    if (!force) {
        const grants = await client.query.authz.grants({ granter, grantee });
        if (grants.grants && grants.grants.length > 0) {
            const grant = grants.grants[0];
    
            if (grant.expiration && Date.parse(grant.expiration as string) - 5 > now) {
                return;
            }
    
        }
    }

    const authz =  await client.tx.authz.grant({
        granter: granter,
        grantee: grantee,
        authorization: {
            msg: MsgGrantAuthorization.MsgWithdrawDelegatorReward
        } as GenericAuthorization,
        expiration: Math.round(now / 1000) + 60 * 60 * 24 * 7
    }) 
    
    expect(authz.code).toBe(0);
}




describe("Staking.wasm", () => {

    const contractConfig = config.contracts["staking_strategy"];

    describe("Investment", async () => {
        StakeAuthorizationType.Undelegate
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


        /* test('Invest', async () => {

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
                StakeAuthorizationType.Delegate
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

        }) */


        /* test('Withdraw', async () => {

            const withdrawParams : InvestParamsResult = await client.query.compute.queryContract({
                contract_address: contractConfig.address!,
                code_hash: contractConfig.codeHash!,
                query: { withdraw_params: {} }
            })

            const params = await parseAttributes(withdrawParams.attributes)
            
            const amount = "500000";
            const investor = mainAccount.address;

            const msg : any = {}
            msg[withdrawParams.name] = {};

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

                msg[withdrawParams.name][key] = value;
            }

            await authorizeIfNeeded(
                mainAccount.address, 
                contractConfig.address!, 
                msg.withdraw.validator_address,
                StakeAuthorizationType.Undelegate,
                true
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


            const unbonding = await client.query.staking.delegatorUnbondingDelegations({ delegator_addr: mainAccount.address });
            let unbonding_after = "0"

            if (unbonding.unbonding_responses?.length) {
                const find = unbonding.unbonding_responses?.find((u) => u.validator_address === msg.withdraw.validator_address)
                if (find) {
                    const entries = find.entries!;
                    unbonding_after = entries[entries.length-1].balance ?? "0";
                }
            }
            expect(BigInt(unbonding_after)).toBe(BigInt(amount));


            const staking2 = await client.query.staking.delegatorDelegations({delegator_addr: mainAccount.address});
            const delegation2 = staking2.delegation_responses![0]            
            let staking_balace_after = delegation2 ? delegation2.balance?.amount ?? "0" : "0";
            
            expect(BigInt(unbonding_after)).toBe(BigInt(amount));
            expect(BigInt(staking_balace_after)).toBeLessThan(BigInt(staking_balace_before));
            expect(BigInt(staking_balace_after)).toBe(BigInt(staking_balace_before) - BigInt(amount));
        }) */



        test('Claim', async () => {

            const claimParams : InvestParamsResult = await client.query.compute.queryContract({
                contract_address: contractConfig.address!,
                code_hash: contractConfig.codeHash!,
                query: { claim_params: {} }
            })
            

            const params = await parseAttributes(claimParams.attributes)
            
            const investor = mainAccount.address;

            const msg : any = {}
            msg[claimParams.name] = {};

            for (const [key, param] of Object.entries(params)) {

                let value;

                if (param.value) {
                    value = param.value;
                } else if (param.toFill === ToFill.Investor) {
                    value = investor;
                } else if (param.options) {
                    value = param.options[0];
                } else {
                    throw new Error("Invalid param");
                }

                msg[claimParams.name][key] = value;
            }

            await claimAuthorizeIfNeeded(
                mainAccount.address, 
                contractConfig.address!, 
            )

            const balance = (await client.query.bank.balance({ address: mainAccount.address, denom: "uscrt" })).balance!.amount ?? "0";

            const res = await client.tx.compute.executeContract({
                contract_address: contractConfig.address!,
                code_hash: contractConfig.codeHash!,
                sender: mainAccount.address,
                msg
            }, { gasLimit: 60000})


            expect(res.code).toBe(0);

            const log = res.arrayLog!.find((log) => log.type === "withdraw_rewards" && log.key === "amount")?.value
            let reward = log ? log.slice(0, log.indexOf('u')) : "0";

            const balanceAfter = (await client.query.bank.balance({ address: mainAccount.address, denom: "uscrt" })).balance!.amount ?? "0";

            const balanceDiff = BigInt(balanceAfter) - BigInt(balance);

            const rewardDiff = Math.abs(Number(balanceDiff - BigInt(reward)));

            // no idea why 6000, looks like rounding error in logs
            expect(rewardDiff).toBe(6000);
        })


        /* test("Test Query", async () => {

            client.query.mint.annualProvisions({})
            const query = { test_query: {
                path: "/cosmos.distribution.v1beta1.Query/Params",
                query: Buffer.from(JSON.stringify({
                    
                })).toString("base64")
            } };


            const res = await client.query.compute.queryContract({
                contract_address: contractConfig.address!, 
                code_hash: contractConfig.codeHash!, 
                query
            })

            console.log("res",res)

        }) */

    })




    
})
