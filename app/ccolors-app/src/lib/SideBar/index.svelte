<script lang="ts">
	const colorCategories = [
		{ name: 'Red', code: 'text-red-600' },
		{ name: 'Orange', code: 'text-yellow-700' },
		{ name: 'Yellow', code: 'text-yellow-500' },
		{ name: 'Green', code: 'text-green-600' },
		{ name: 'Blue', code: 'text-blue-700' },
		{ name: 'Purple', code: 'text-purple-600' },
		{ name: 'Brown', code: 'text-yellow-900' },
		{ name: 'Gray', code: 'text-gray-500' }
	];
</script>

<div id="sidebar">
	<button class="rainbow"> Random Color </button>
	<ul class="mt-6 w-4/5 h-10 mx-auto">
		{#each colorCategories as { name, code }}
			<li class="text-xl my-2">
				<a class="transition-colors duration-500 ease-in-out hover:underline hover:{code}" href="/"
					>{name}</a
				>
			</li>
		{/each}
	</ul>
	<!-- This is just to get around Tailwind/PostCSS tree shaking to make sure the hover: styles get bundled -->
	<span
		class="hidden hover:text-red-600
                hover:text-yellow-700
                hover:text-yellow-500
                hover:text-green-600
                hover:text-blue-700
                hover:text-purple-600
                hover:text-yellow-900
                hover:text-gray-500"
	/>
</div>

<style>
	#sidebar {
		background-color: #d6d8d8;
		box-shadow: 2px 0px 6px rgba(0, 0, 0, 0.2);
		color: #363c3c;
	}
	.rainbow:hover {
		/* background-color: transparent; */
		background: linear-gradient(
			45deg,
			#ff000060,
			#ff730060,
			#fffb0060,
			#48ff0060,
			#00ffd560,
			#002bff60,
			#7a00ff60,
			#ff00c860,
			#ff000060
		);
		background-size: 350%;
		animation: glowing 20s linear infinite;
		border-radius: 10px;
	}
	@keyframes glowing {
		0% {
			background-position: 0 0;
		}
		50% {
			background-position: 400% 0;
		}
		100% {
			background-position: 0 0;
		}
	}
</style>
