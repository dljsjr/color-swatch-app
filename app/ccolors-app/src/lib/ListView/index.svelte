<script lang="ts">
    export let colors;
</script>

<div class="h-full w-full">
    {#each colors as {name, value }, i}
        <p>Color named {name} has hsv ({value.hue}, {value.sat}, {value.val})</p>
    {/each}
</div>