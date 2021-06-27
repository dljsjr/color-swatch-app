<script lang="ts">
	import SwatchPreview from '$lib/SwatchPreview/index.svelte';
	export let colors;

	export const getHSLSatFromHSV = (value: { hue: number; sat: number; val: number }): number => {
		return (
			(value.sat * value.val) /
			((value.hue = (2 - value.sat) * value.val) < 1 ? value.hue : 2 - value.hue)
		);
	};
</script>

<div class="container mx-auto px-10 flex flex-wrap flex-row overflow-auto">
	{#each colors as { name, value }}
		<SwatchPreview
			class="my-8 mx-auto md:mx-3"
			colorName={name}
			hsl={[360 * value.hue, 100 * getHSLSatFromHSV(value), (100 * value.hue) / 2]}
		/>
	{/each}
</div>
