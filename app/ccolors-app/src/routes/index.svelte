<script lang="ts" context="module">
	import { totalColors } from '$lib/stores/state';
	/**
	 * @type {import('@sveltejs/kit').Load}
	 */
	export async function load({ page, fetch, session, context }) {
		const metadata_fetch = await fetch('/api/count');

		if (metadata_fetch.ok) {
			const count = (await metadata_fetch.json())['json']['count'];
			totalColors.set(count);
		} else {
			return {
				status: metadata_fetch.status,
				error: new Error(`Error getting color list from API ${metadata_fetch.error}`)
			};
		}

		const colors_fetch = await fetch('/api/colors/1');
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

<ListView colors={json} />
