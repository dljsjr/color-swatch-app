<script lang="ts">
	import { hslToRgb, rgbToHexString } from '$lib/colors';

	export let colorName: string = '';
	export let hsl: [number, number, number] = [0.0, 0.0, 0.0];

	$: hexCode = (): string => {
		return rgbToHexString(...hslToRgb(...hsl));
	};
</script>

<div
	class="swatch-preview {$$props.class}"
	style="background-color: hsl({hsl[0]}, {hsl[1]}%, {hsl[2]}%);"
>
	<div class="w-full h-5/6" />
	<p class="text-base pl-2 pb-0.5 overflow-ellipsis overflow-hidden w-full text-left bg-white">
		{hexCode()}
	</p>
	<p
		class="text-xs pl-2 pb-1 overflow-ellipsis overflow-hidden w-full text-left bg-white rounded-b-md"
	>
		{colorName}
	</p>
</div>

<style lang="scss">
	.swatch-preview {
		box-shadow: 0px 0px 6px rgba(0, 0, 0, 0.5);
		border: 1px solid #fdfdfd;
		p {
			color: #363c3c;
			background-color: #fdfdfd;
		}
	}
</style>
