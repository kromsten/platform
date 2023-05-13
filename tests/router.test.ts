import type { RouterStrategy } from '$interfaces/contracts';
import { expect, test, describe, it} from 'vitest';
import { getAccount } from './accounts';
import configuration from './config';
import type { ContractConfig} from './interfaces';
import { contractExecutor, contractQuerier, msgMapping } from './utils';

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


    describe("Adding route is working", async () => {

        /* await executeContract({ 
            add_strategy: { 
                contract: {
                    address: strategy.address!,
                    hash: strategy.codeHash!,
                }
            }
        }); */

        const strategies : RouterStrategy[] = await queryContract({ 
            token_strategies: { token: { native: "uscrt" } 
        }})
        
        expect(Array.isArray(strategies)).toBe(true);
        expect(strategies.length).toBeGreaterThan(0);



        describe("each strategy", () => {
            
            const first = strategies[0];

            const queryStrategy = contractQuerier(client, first.contract.address, first.contract.hash)
       
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
    
    
            it("should return strategy info", async () => {

                for (const s of strategies) {

                    const queryStrategy = contractQuerier(client, s.contract.address, s.contract.hash)
                    const info : any = await queryStrategy({ strategy_info : {} })
                    console.log("info: ", info)

                    for (const msg of info.invest_msgs) {
                        expect(msgMapping[msg.type_url]).toBeDefined();
                    }

                }

    
            })

        });

    });


});
