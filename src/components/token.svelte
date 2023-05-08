<script lang="ts">
	import { supportedTokens } from "../config";
	import { logoMapping } from "../config/logos";
    export let token : string;
    const tokenData = supportedTokens[token];
</script>


<div class="flex gap-2 align-center">
    <svelte:component this={logoMapping[tokenData.symbol]} />
    {tokenData.symbol}
</div>