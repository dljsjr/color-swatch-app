<script lang="ts" context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const url = '/api/search?' + page.query.toString();

		const colorSearchResponse = await fetch(url);

		if (colorSearchResponse.ok) {
			return {
				props: {
					json: (await colorSearchResponse.json())['json']
				}
			};
		} else {
			return {
				status: colorSearchResponse.status,
				error: new Error(`Error getting color list from API ${colorSearchResponse.error}`)
			};
		}
	}
</script>

<script lang="ts">
	import ListView from '$lib/ListView/index.svelte';

	export let json;
</script>

<ListView colors={json} paginate={false} />
