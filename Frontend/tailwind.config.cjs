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
      },
      height:
      {
        "btnHei":"66px"
      },
      colors:
      {
        "btnCol":"#124E86",
        "btnTxt": "white"

      }

    },
  },
  plugins: [
    require("daisyui")
  ],
  
}