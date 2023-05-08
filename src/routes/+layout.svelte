<script lang='ts'>
	// The ordering of these imports is critical to your app working properly
	import '@skeletonlabs/skeleton/themes/theme-rocket.css';
	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/all.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	import { init } from '$lib';
	import { formatAddress } from '$lib/utils';
	import { PUBLIC_SCRT_CHAIN_ID } from '$env/static/public';
	import { connectSecret } from '$lib/connector';
	import { networksState, tokensState } from '$lib/state';

	$: secretAddress = $networksState[PUBLIC_SCRT_CHAIN_ID]?.address

	//$: console.log('netwokrs', $networksState);
	//$: console.log('tokens', $tokensState);

	onMount(init);
</script>



<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar>
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">Skeleton</strong>
			</svelte:fragment>
			<svelte:fragment slot="trail">

			{ #if secretAddress }
				<span>
					<span class="text-sm font-bold">{ formatAddress(secretAddress) }</span>
				</span>
			{ :else }
				<button
					class="btn btn-sm variant-ghost-surface"
					on:click={() => connectSecret()}
				>
					Connect
				</button>
			{/if}

			</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<!-- Page Route Content -->
	<slot />
</AppShell>
