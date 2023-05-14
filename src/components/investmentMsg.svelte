<script lang="ts">
	import type { InvestmentAction } from "$interfaces/investments";
	import { renderCosmosType } from "$lib/utils";
    export let msg: InvestmentAction


    export let parentRect : DOMRect;
    let elem: HTMLDivElement

    let transformatinClasses = "";

    $: if (parentRect && elem) {
        
        const ownRect = elem.getBoundingClientRect();
        const parentCenter = parentRect.top + (parentRect.height / 2) - parentRect.height / 7;
        
        const withinParentCenter = (ownRect.top <= parentCenter && ownRect.bottom >= parentCenter)


        if (withinParentCenter) {
            transformatinClasses = "--tw-scale-x: 1; --tw-scale-y: 1;";
        } else {
            const fromTop = ownRect.bottom < parentCenter
            const distance = 
                    fromTop 
                    ? (parentCenter - ownRect.bottom)
                    : (ownRect.top - parentCenter);
            

            const scale =  Math.max(Math.min(elem.clientHeight / distance, 0.95), 0.1)
            const rawTranslate = 8 * distance / elem.clientHeight 
            const translate =   Math.max(Math.min(rawTranslate, 50), 0.1)
            const rotate =  2 * Math.max(Math.min(rawTranslate, 50), 0.1)

            transformatinClasses = `--tw-scale-x: ${scale.toFixed(2)}; --tw-scale-y: ${scale.toFixed(2)}; --tw-translate-x: ${Math.round(translate)}rem; --tw-rotate: ${Math.round(fromTop ? rotate : -rotate)}deg`
        }
        
        
    }
   

</script>


<div bind:this={elem} class="shadow-xl rounded-md p-4 h-44 w-72 snap-center snap-normal hover:scale-150 flex flex-col gap-2  transform {msg.exposes_investor ? 'bg-white' : ''}" style={transformatinClasses}>
    <div class="flex justify-between">
        <span class="font-bold text-primary-500">Type:</span>
        <span class="text-ellipsis overflow-hidden">{ renderCosmosType(msg.type_url) }</span>
    </div>

    <div class="flex justify-between">
        <span class="font-bold text-primary-500">Chain Id:</span>
        <span>{ msg.chain_id }</span>
    </div>

    <div class="flex justify-between">
        <span class="font-bold text-primary-500">Description:</span>
        <span>{ msg.description }</span>
    </div>
</div>

