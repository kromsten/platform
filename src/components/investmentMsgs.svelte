<script lang="ts">
	import InvestmentMsg from "./investmentMsg.svelte";
	import { investmentMessages } from "$lib/state";


	let elem : HTMLElement, rect: DOMRect;

	const onScroll = () => {
		rect = elem.getBoundingClientRect();
		console.log("\n\n")
	}
</script>


<section bind:this={elem} class="overflow-x-hidden bg-surface-500/20 border rounded-md border-primary-500/50 overflow-y-scroll h-full scroll-smooth scroll-py-5 scroll-my-5 w-full" on:scroll={onScroll}>
	<div class="flex flex-col gap-y-10 px-3 ps-5 pt-5 pe-40">
		{ #each $investmentMessages as msg, i (i)}
			{ #if i == 0}
				<div class="h-5"></div>
			{/if}
			<InvestmentMsg {msg} parentRect={rect} />
			{ #if i == $investmentMessages.length - 1}
				<br class="h-10" />
			{/if}
		{/each }
	</div>
</section>

