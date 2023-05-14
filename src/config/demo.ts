import { PUBLIC_SCRT_CHAIN_ID } from "$env/static/public";
import type { RouterStrategy } from "$interfaces/contracts";
import { ActionClass, type InvestmentAction } from "$interfaces/investments";

export const mockStrategies : {[ token: string] : RouterStrategy[]} = {
    uscrt: [
        {
            contract: {
                address: "1",
                hash: ""
            },
            inputs: [ { native: 'uscrt' } ],
            outputs: [ { native: 'uscrt' } ],
            demo: true
        }
    ]
}


export const mockStrategyMessages : {[ strategy: string] : InvestmentAction[]} = {
    "1": [
        {
            type_url: "/cosmos.staking.v1beta1.MsgDelegate",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.MsgUndelegate",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Lol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Bol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Gol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Dol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Mol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Col",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Lol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Bol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Gol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Dol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Mol",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        },

        {
            type_url: "/cosmos.staking.v1beta1.Col",
            chain_id: PUBLIC_SCRT_CHAIN_ID,
            exposes_investor: true,
            attributes: [],
            optional: false,
            class: ActionClass.Staking,
            action_requirements: [],
            issued_tokens: [],
        }
    ]
};