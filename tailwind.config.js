/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
daisyui: {
	themes: [
			{
				indy: {
					"primary": "#FA5D89",
					"accent": "#FEF7EC",
					"secondary": "#2E4254",
					"neutral": "#222C36",
				}
			}
		]
},
  theme: {
    extend: {},
  },
  plugins: [
		require('daisyui')
	],
}
