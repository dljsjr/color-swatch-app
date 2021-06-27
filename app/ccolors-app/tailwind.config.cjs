const config = {
	mode: "jit",
	purge: [
		"./src/**/*.{html,js,svelte,ts}",
	],
	theme: {
		extend: {
			fontFamily: {
				'source-serif-pro': ['"Source Serif Pro"', 'serif']
			}
		},
	},
	plugins: [],
};

module.exports = config;
