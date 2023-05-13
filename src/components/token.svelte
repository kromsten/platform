<script lang="ts">
	import { supportedTokens } from "../config";
	import { logoMapping } from "../config/logos";

    import {tokensState} from "$lib/state"
	import { formatBalance } from "$lib/utils";

    export let token : string;
    const tokenData = supportedTokens[token];
    $: tokenState  = $tokensState[token];
</script>


<div class="flex gap-2 align-center justify-between">
    <div class="flex gap-2 align-center">
        <svelte:component this={logoMapping[tokenData.symbol]} />
        {tokenData.symbol}
    </div>

    { #if tokenState }
        <span>{ formatBalance(token, tokenState)[1]}</span>
    { /if }
</div>