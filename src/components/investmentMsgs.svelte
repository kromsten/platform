<script lang="ts">
	import { investmentMessages } from "$lib/state";
	import InvestmentMsg from "./investmentMsg.svelte";
</script>


{ #each $investmentMessages as msg}
    <InvestmentMsg {msg} />
{/each }