<script lang="ts">
	import { supportedTokens } from "../config";
    import { ListBox, ListBoxItem, popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import {selectedToken} from "$lib/state"
	import Token from "./token.svelte";

    import { crossfade } from "svelte/transition";
    import { quintOut } from 'svelte/easing';


    const tokens  = Object.keys(supportedTokens)

    const onSelect = (e : any) => {
        selectedToken.set(e.target.value);
    }

    const popupCombobox: PopupSettings = {
        event: 'focus-click',
        target: 'tokens',
        placement: 'bottom',
        closeQuery: '.listbox-item'
    };



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

    let selected : string | undefined = undefined;
    
    $: if ($selectedToken) setTimeout(() => selected = $selectedToken, 1000)

</script>


<article class="mx-auto flex h-full items-center">
    


    <button 
        in:receive="{{key: $selectedToken}}"
        out:send="{{key: $selectedToken}}"
        class="btn variant-filled w-48" 
        use:popup={popupCombobox}
    >
            <span class="capitalize">{selected ?? 'Select Token'}</span>
    </button>

    
    
    <div class="card w-48 shadow-xl py-2" data-popup="tokens">
        <ListBox rounded="rounded-none">
            { #each tokens as token (token)}
                <ListBoxItem bind:group={$selectedToken} name="token" value={token} on:change={onSelect}>
                    <Token {token} />
                </ListBoxItem>
            { /each }
        </ListBox>
    </div>

</article>

