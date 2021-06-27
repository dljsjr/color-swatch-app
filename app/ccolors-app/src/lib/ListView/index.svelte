<script lang="ts">
	import SwatchPreview from '$lib/SwatchPreview/index.svelte';
	import { totalPages } from '$lib/stores/state';

	export let colors;

	export const getHSLSatFromHSV = (value: { hue: number; sat: number; val: number }): number => {
		return (
			(value.sat * value.val) /
			((value.hue = (2 - value.sat) * value.val) < 1 ? value.hue : 2 - value.hue)
		);
	};
</script>

<div class="container mx-auto flex flex-col overflow-auto">
	<div class="container mx-auto px-10 flex flex-wrap flex-row">
		{#each colors as { name, value }}
			<SwatchPreview
				class="my-8 mx-auto md:mx-3"
				colorName={name}
				hsl={[360 * value.hue, 100 * getHSLSatFromHSV(value), (100 * value.hue) / 2]}
			/>
		{/each}
	</div>
	<div class="flex flex-row justify-center">
		{#each Array($totalPages) as _, idx}
			<a class="text-2xl mx-2 mt-8 hover:underline hover:text-black" href="/colors/{idx + 1}">
				{idx + 1}
			</a>
		{/each}
	</div>
</div>

<style>
	a {
		color: #363c3c;
	}
</style>
