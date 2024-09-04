/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./node_modules/flowbite/**/*.js", "./*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('flowbite/plugin')
  ],
}

