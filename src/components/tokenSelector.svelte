<script lang="ts">
	import { supportedTokens } from "../config";
    import { ListBox, ListBoxItem, popup, type PopupSettings } from '@skeletonlabs/skeleton';
    import {selectedToken} from "$lib/state"
	import Token from "./token.svelte";

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


</script>


<article class="mx-auto flex h-full items-center">
    <button class="btn variant-filled w-48" use:popup={popupCombobox}>
        <span class="capitalize">{$selectedToken ?? 'Select Token'}</span>
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

