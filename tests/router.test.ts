import { expect, test, describe, it} from 'vitest';
import { getAccount } from './accounts';
import configuration from './config';
import type { ContractConfig, Strategy } from './interfaces';
import { contractExecutor, contractQuerier } from './utils';

const account = getAccount();
const client = account.secretjs;




describe('Routes', () => {

    let config : ContractConfig = configuration.contracts["router"];
    expect(config).toBeDefined();
    expect(config.codeId).toBeDefined();
    expect(config.address).toBeDefined();


    
    const queryContract = contractQuerier(client, config.address!, config.codeHash!);
    const executeContract = contractExecutor(client, config.address!, config.codeHash!);


    const strategy = configuration.contracts["staking_strategy"];


    test("Adding route is working", async () => {

        /* await executeContract({ 
            add_strategy: { 
                contract: {
                    address: strategy.address!,
                    hash: strategy.codeHash!,
                }
            }
        }) */

        const strategies : Strategy[] = await queryContract({ 
            token_strategies: { token: { native: "uscrt" } 
        }})
        
        console.log("Strategies:", strategies);

        expect(Array.isArray(strategies)).toBe(true);
        expect(strategies.length).toBeGreaterThan(0);

        test("each strategy", () => {

            it("should have at least one input and output", () => {
                strategies.forEach(s => {
                    expect(s.inputs.length).toBeGreaterThan(0)
                    expect(s.outputs.length).toBeGreaterThan(0)
                })
            })

            it("should have have uscrt as an input", () => {
                expect(strategies.findIndex(s => 
                    s.inputs.findIndex(inp =>  'native' in inp && inp.native === "uscrt") >= 0
                )).toBeGreaterThan(-1)
            })
        });

    });


});
