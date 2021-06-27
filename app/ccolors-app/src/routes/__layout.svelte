<script>
	import '../app.postcss';
	import Header from '$lib/Header/index.svelte';
	import SideBar from '$lib/SideBar/index.svelte';
</script>

<div class="h-screen w-screen flex flex-col">
	<Header />
	<div class="h-full w-full flex flex-row">
		<SideBar />
		<slot />
	</div>
</div>
