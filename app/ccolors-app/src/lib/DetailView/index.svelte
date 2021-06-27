<script lang="ts">
	import { hslToRgb, rgbToHexString } from '$lib/_colors';

	export let colorName: string = '';
	export let hsl: [number, number, number] = [0.0, 0.0, 0.0];
	export let id: number = 0;

	$: hexCode = (): string => {
		return rgbToHexString(...hslToRgb(...hsl));
	};
</script>

<div
	class="swatch-detail justify-end {$$props.class}"
	style="background-color: hsl({hsl[0]}, {hsl[1]}%, {hsl[2]}%);"
	href="/detail/{id}"
>
	<div class="w-full h-5/6" />
	<p class="large-hex-code">
		{hexCode()}
	</p>
	<p class="large-color-name">
		{colorName}
	</p>
</div>

<style lang="scss">
	.swatch-detail {
		box-shadow: 0px 2px 6px rgba(0, 0, 0, 0.2);
		box-shadow: inset 0px 2px 6px rgba(0, 0, 0, 0.2);
		p {
			color: #363c3c;
			background-color: #fdfdfd;
		}
	}
</style>
