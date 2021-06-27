<script lang="ts" context="module">
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const api_url = 'http://localhost:8000/colors?limit=25&start_at=1';
		const api_response = await fetch(api_url);

		if (api_response.ok) {
			return {
				props: {
					json: await api_response.json()
				}
			};
		}

		return {
			status: api_response.status,
			error: new Error(`Error getting color list from API ${api_response.error}`)
		};
	}
</script>

<script lang="ts">
	import ListView from '$lib/ListView/index.svelte';

	export let json;
</script>

<ListView colors={json} />
