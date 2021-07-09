<script lang="ts" context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		let color_family = page.params.family;
		const colors_fetch = await fetch(`/api/search/family/${color_family}`);
		let colors_json = (await colors_fetch.json())['json'];

		if (colors_fetch.ok) {
			return {
				props: {
					json: colors_json
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
	import ListView from '$lib/ListView/index.svelte';

	export let json;
</script>

<ListView colors={json} paginate={false} />
