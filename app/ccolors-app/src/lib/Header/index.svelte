<script lang="ts">
	import { goto } from '$app/navigation';

	let searchString = '';

	const ENTER_CHAR_CODE = 13;

	const onKeyPress = async (e) => {
		if (e.charCode === ENTER_CHAR_CODE) doSearch(searchString);
	};

	const doSearch = async (searchString: string) => {
		const params: URLSearchParams = new URLSearchParams();
		params.set('text', searchString);

		const url = '/search/results?' + params.toString();

		goto(url);
	};
</script>

<div class="header" id="header">
	<a href="/"
		><img src="/logo-symbol.svg" alt="Helpful Humans Logo" class="block h-4/5 w-14 ml-8" /></a
	>
	<input
		on:keypress={onKeyPress}
		bind:value={searchString}
		placeholder="Search"
		class="h-2/5 w-72 p-4 mr-6 rounded-md"
	/>
</div>

<style>
	#header {
		background-color: #363c3c;
		box-shadow: 0px 2px 6px rgba(0, 0, 0, 0.2);
	}
</style>
