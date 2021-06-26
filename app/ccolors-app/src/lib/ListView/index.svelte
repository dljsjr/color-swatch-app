<script lang="ts">
    export let colors;

    export function getHSLSat(value) {
        return value.sat*value.val/((value.hue=(2-value.sat)*value.val)<1?value.hue:2-value.hue)
    }
</script>

<div class="h-full w-full">
    {#each colors as {name, value }, i}
        <p style="color: hsl(0, {100 * getHSLSat(value)}%, {100 * (value.hue/2)}%);">{name}</p>
    {/each}
</div>