<script lang="ts" context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		let requestedID = parseInt(page.params.id);
		const colors_fetch = await fetch(`/api/detail/${requestedID}`);
		let colors_json = (await colors_fetch.json())['json'];

		if (colors_fetch.ok) {
			return {
				props: {
					color: colors_json
				}
			};
		} else {
			return {
				status: colors_fetch.status,
				error: new Error(`Error getting color list from API ${colors_fetch.error}`)
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
