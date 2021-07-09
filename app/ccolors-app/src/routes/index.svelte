<script lang="ts" context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const colorsFetch = await fetch('/api/colors/1');
		let colorsJson = (await colorsFetch.json())['json'];

		if (colorsFetch.ok) {
			return {
				props: {
					json: colorsJson
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
	import ListView from '$lib/ListView/index.svelte';

	export let json;
</script>

<ListView colors={json} paginate={true} />
