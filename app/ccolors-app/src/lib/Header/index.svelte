<script>
	let searchString = '';
</script>

<div class="header" id="header">
	<img src="/logo-symbol.svg" alt="Helpful Humans Logo" class="block h-4/5 w-14 ml-6" />
	<input bind:value={searchString} placeholder="Search" class="h-2/5 w-72 p-4 mr-6 rounded-md" />
</div>

<style>
	#header {
		background-color: #363c3c;
		box-shadow: 0px 2px 6px rgba(0, 0, 0, 0.2);
	}
</style>
