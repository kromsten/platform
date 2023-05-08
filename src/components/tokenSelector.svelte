<script lang="ts">
	import { supportedTokens } from "../config";
    import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
    import {selectedToken} from "$lib/state"
	import Token from "./token.svelte";

    const tokens  = Object.keys(supportedTokens)

    const onSelect = (e : any) => {
        selectedToken.set(e.target.value);
    }

</script>


<div class="mx-auto flex h-full items-center">
    <ListBox >
        { #each tokens as token (token)}
            <ListBoxItem bind:group={$selectedToken} name="token" value={token} on:change={onSelect}>
                <Token {token} />
            </ListBoxItem>
        {/each}
    </ListBox>
</div>