<script lang="ts">
	import { currentStrategies, selectedToken } from "$lib/state";
	import { quintOut } from "svelte/easing";
	import { crossfade } from "svelte/transition";

    import Strategy from "$components/strategy.svelte";
	import Token from "$components/token.svelte";
	import { fade, fly  } from 'svelte/transition';

     const [send, receive] = crossfade({
		duration: d => Math.sqrt(d * 200),

		fallback(node, params) {
			const style = getComputedStyle(node);
			const transform = style.transform === 'none' ? '' : style.transform;

			return {
				duration: 600,
				easing: quintOut,
				css: t => `
					transform: ${transform} scale(${t});
					opacity: ${t}
				`
			};
		}
	});

    $: token = $selectedToken!
</script>


<div class="flex p-2" 
    in:receive="{{key: $selectedToken}}"
    out:send="{{key: $selectedToken}}"
>
    <Token {token} />
</div>


<div class="flex gap-4 mt-5" in:fly="{{ y: 200, duration: 2000 }}">
    { #each $currentStrategies as strategy (strategy.contract) }
        <Strategy {strategy}/>
    {/each}
</div>