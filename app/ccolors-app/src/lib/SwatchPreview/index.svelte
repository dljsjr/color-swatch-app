<script lang="ts">
	import { hslToRgb, rgbToHexString } from '$lib/_colors';

	export let colorName: string = '';
	export let hsl: [number, number, number] = [0.0, 0.0, 0.0];
	export let id: number = 0;

	$: hexCode = (): string => {
		return rgbToHexString(...hslToRgb(...hsl));
	};
</script>

<a
	class="swatch-preview {$$props.class}"
	style="background-color: hsl({hsl[0]}, {hsl[1]}%, {hsl[2]}%);"
	href="/detail/{id}"
>
	<div class="w-full h-5/6" />
	<p class="preview-hex-code">
		{hexCode()}
	</p>
	<p class="preview-color-name">
		{colorName}
	</p>
</a>

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
