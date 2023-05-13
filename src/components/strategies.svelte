<script>
	import { currentStrategies } from "$lib/state";
	import Strategy from "./strategy.svelte";

</script>


<div class="flex gap-4 mt-5">
    { #each $currentStrategies as strategy (strategy.contract) }
        <Strategy {strategy}/>
    {/each}
</div>