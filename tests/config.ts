import nonTypedconfig from "$contractConfig";
import type { Config } from "./interfaces";

export const config : Config = nonTypedconfig as Config
if (!config.contracts) config.contracts = {};

export default config;