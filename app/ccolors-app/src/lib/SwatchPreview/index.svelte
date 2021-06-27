<script lang="ts">
	import { hslToRgb, rgbToHexString } from '$lib/colors';

	export let colorName: string = '';
	export let hsl: [number, number, number] = [0.0, 0.0, 0.0];

	$: hexCode = (): string => {
		return rgbToHexString(...hslToRgb(...hsl));
	};
</script>

<div
	class="flex-grow-0 flex-shrink-0 w-40 h-44 rounded-lg flex flex-col content-start item-start {$$props.class}"
	style="background-color: hsl({hsl[0]}, {hsl[1]}%, {hsl[2]}%);"
>
	<div class="w-full h-5/6" />
	<p class="text-xs overflow-ellipsis overflow-hidden w-full text-center bg-white">{colorName}</p>
	<p class="text-base overflow-ellipsis overflow-hidden w-full text-center bg-white">{hexCode()}</p>
</div>
