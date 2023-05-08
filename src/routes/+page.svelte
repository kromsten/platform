<script lang="ts">
	import Token from "$components/token.svelte";
import TokenSelector from "$components/tokenSelector.svelte";
	import { selectedToken } from "$lib/state";
</script>

<main class="container mx-auto h-full flex p-5">
	
	{ #if $selectedToken }
		<Token token={$selectedToken} />
	{ :else }
		<TokenSelector />
	{/if}

</main>
