module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {
      fontSize:{
        "fillin":"27px",
        "btnTxt":"28px"
      },
      width: 
      {
        "fillin":"378px",
        "btnWid":"336px"
      },
      height:
      {
        "fillin":"43px",
        "btnHei":"66px"
      },
      colors:
      {
        "fillin":"white",
        "btnCol":"#124E86",
        "btnTxt": "white"

      }

    },
  },
  plugins: [
    require("daisyui")
  ],
  
}