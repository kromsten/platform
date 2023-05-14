<script lang="ts">
	import Messages from "$components/states/messages.svelte";
	import Strategies from "$components/states/strategies.svelte";
	import TokenSelector from "$components/tokenSelector.svelte";
	import { selectedStrategy, selectedToken } from "$lib/state";
</script>


<div class="container mx-auto h-full flex flex-col" on:scroll={() => { console.log("scroll") }}>
	
	{ #if $selectedStrategy }
		<Messages />
	{ :else if $selectedToken }
		<Strategies />
	{ :else }
		<TokenSelector />
	{/if}

</div>
