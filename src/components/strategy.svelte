<script lang="ts">
	import type { RouterStrategy } from "$interfaces/contracts";
	import { selectedStrategy } from "$lib/state";
	import { formatAddress } from "$lib/utils";
    export let strategy : RouterStrategy;

    const select = () => {
        selectedStrategy.set(strategy);
    }

</script>


<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-interactive-supports-focus -->
<div class="card bg-red p-3" on:click={select} role="button">
    <h5>Strategy:</h5>
    { formatAddress(strategy.contract.address ?? "") }

</div>

