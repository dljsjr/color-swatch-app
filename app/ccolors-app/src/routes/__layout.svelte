<script lang="ts" context="module">
	import { totalColors, colorsPerPage, totalPages } from '$lib/stores/state';
	export async function load({ page, fetch, session, context }) {
		const metadataFetch = await fetch('/api/count');

		if (metadataFetch.ok) {
			let limit: number;
			colorsPerPage.subscribe((val) => (limit = val))();

			const count = (await metadataFetch.json())['json']['count'];
			totalColors.set(count);

			const pages = Math.ceil(count / limit);
			totalPages.set(pages);

			return {
				status: 200
			};
		} else {
			return {
				status: metadataFetch.status,
				error: new Error(`Error getting color list from API ${metadataFetch.error}`)
			};
		}
	}
</script>

<script>
	import '../app.postcss';
	import Header from '$lib/Header/index.svelte';
	import SideBar from '$lib/SideBar/index.svelte';
</script>

<div class="h-full w-full flex flex-col">
	<Header />
	<div class="h-full w-full flex flex-row overflow-hidden">
		<SideBar />
		<slot />
	</div>
</div>
