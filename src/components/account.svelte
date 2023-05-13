<script lang="ts">
	import { connectSecret } from "$lib/connector";
	import { networksState, tokensState, walletName } from "$lib/state";
	import { formatAddress, formatBalance } from "$lib/utils";

    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';

    const popupSettings: PopupSettings = {
        event: 'click',
        target: 'popupCombobox',
        placement: 'left',
        closeQuery: '.listbox-item'
    };  

    $: networks = Object.entries($networksState).filter(n => n[1].connected);
    $: tokens = Object.entries($tokensState).map(t => formatBalance(...t));
    $: itemLength = Math.max(networks.length, tokens.length);

    $: console.log("networks:", Object.entries($networksState))
    $: console.log("tokens:", Object.entries($tokensState))
</script>

{ #if $walletName }
    <button
        class="btn btn-sm variant-ghost-primary text-sm font-bold text-primary-700 capitalize"
        use:popup={popupSettings}
    >
        { $walletName }
    </button>

    <div class="card md:w-52 shadow-xl py-2 px-1 bg-primary-200" data-popup="popupCombobox">        

        <div class="flex md:flex-col px-1 gx-8">

            { #if networks.length }

                <section>
                    <h6 class="text-primary-700 capitalize font-bold pl-1 me-5 md:me-0">Networks:</h6>
                    <ul class="list text-xs">
                        { #each networks as network (network[0])}
                            <li class="flex justify-between pt-0 pb-0">
                                <span class="font-bold">{network[0]}</span>
                                <span>{formatAddress(network[1].address, 4)}</span>
                            </li>
                        {/each}
                    </ul>
                </section>

            { /if }

            <span class="md:hidden divider-vertical h-{itemLength * 10} mx-3 self-center" />

            { #if networks.length }

                <section>
                    <h6 class="text-primary-700 capitalize font-bold pl-1 md:mt-2">Tokens:</h6>
                    <ul class="list text-xs">
                        { #each tokens as token (token[0])}
                            <li class="flex  justify-between">
                                <span class="font-bold">{token[0]}</span>
                                <span>{token[1]}</span>
                            </li>
                        {/each}
                    </ul>

                </section>


            { /if }
            
        </div>

        
        <div class="arrow bg-surface-100-800-token" />
    </div>

    <!-- <div class="card bg-primary-200" data-popup="popupCombobox">

        <dl class="list-dl flex flex-row">

            <div class="flex flex-row">
                <dt>Networks:</dt>

                <dd class="flex flex-row">
                    <ul class="list-dl">

                        { #each networks as network (network[0])}
                            <dt>{network[0]}</dt>
                            <dd>{formatAddress(network[1].address)}</dd>
                        {/each}
                    </ul>
                </dd>

            </div>



        </dl>


        

    </div> -->


{ :else }
    <button
        class="btn btn-sm variant-ghost-primary text-sm font-bold text-primary-700"
        on:click={() => connectSecret()}
    >
        Connect
    </button>
{/if}