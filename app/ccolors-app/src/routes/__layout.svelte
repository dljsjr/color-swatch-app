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
