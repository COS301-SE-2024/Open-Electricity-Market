module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {
      fontSize:{
        "heading1": "50px",
        "heading2": "33px",
        "fillin":"27px",
        "btnTxt":"28px"
      },
      fontWeight:{
        "fillin":"600"
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
        "fillinText": "black",
        "btnCol":"#124E86",
        "btnTxt": "white"

      }

    },
  },
  plugins: [
    require("daisyui")
  ],
  
}