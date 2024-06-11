module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {
      fontSize:{
        "btnTxt":"28px"
      },
      width: 
      {
        "btnWid":"336px"
      }
    },
  },
  plugins: [
    require("daisyui")
  ],
  
}