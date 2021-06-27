<script lang="ts" context="module">
	import { totalColors, colorsPerPage, totalPages } from '$lib/stores/state';
	export async function load({ page, fetch, session, context }) {
		const metadata_fetch = await fetch('/api/count');

		if (metadata_fetch.ok) {
			let limit: number;
			colorsPerPage.subscribe((val) => (limit = val))();

			const count = (await metadata_fetch.json())['json']['count'];
			totalColors.set(count);

			const pages = Math.ceil(count / limit);
			totalPages.set(pages);

			return {
				status: 200
			};
		} else {
			return {
				status: metadata_fetch.status,
				error: new Error(`Error getting color list from API ${metadata_fetch.error}`)
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
