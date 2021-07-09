<script lang="ts" context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		let requestedID = parseInt(page.params.id);
		const colorsFetch = await fetch(`/api/detail/${requestedID}`);
		let colorsJson = (await colorsFetch.json())['json'];

		if (colorsFetch.ok) {
			return {
				props: {
					color: colorsJson
				}
			};
		} else {
			return {
				status: colorsFetch.status,
				error: new Error(`Error getting color list from API ${colorsFetch.error}`)
			};
		}
	}
</script>

<script lang="ts">
	import DetailView from '$lib/DetailView/index.svelte';

	export let color;

	export const getHSLSatFromHSV = (value: { hue: number; sat: number; val: number }): number => {
		return (
			(value.sat * value.val) /
			((value.hue = (2 - value.sat) * value.val) < 1 ? value.hue : 2 - value.hue)
		);
	};
</script>

<DetailView
	colorName={color.name}
	id={color.id}
	hsl={[360 * color.value.hue, 100 * getHSLSatFromHSV(color.value), (100 * color.value.hue) / 2]}
/>
