<script lang="ts">
	import Messages from "$components/states/messages.svelte";
	import Strategies from "$components/strategies.svelte";
	import Token from "$components/token.svelte";
	import TokenSelector from "$components/tokenSelector.svelte";
	import { selectedToken } from "$lib/state";
</script>


<main class="container mx-auto h-full flex flex-col p-5 pt-8">
	
	{ #if $selectedToken }
		<Token token={$selectedToken} />
	{ :else }
		<TokenSelector />
	{/if}

	<Strategies />
	<Messages />
</main>
