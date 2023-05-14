<script lang='ts'>
	// The ordering of these imports is critical to your app working properly
	
	//import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	import '../theme.postcss';

	// If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
	import '@skeletonlabs/skeleton/styles/all.css';
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';
	import { AppShell, AppBar, autoModeWatcher } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';

	import { init } from '$lib';
	import Account from '$components/account.svelte';

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	onMount(init);
</script>

<svelte:head>{@html `<script>${autoModeWatcher.toString()} autoModeWatcher();</script>`}</svelte:head>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar background="">
			
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase"></strong>
			</svelte:fragment>

			
			<svelte:fragment slot="trail">
				<Account />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<slot />

</AppShell>
