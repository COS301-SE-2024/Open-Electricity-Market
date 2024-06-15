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
        "heading1":"normal",
        "heading2": "normal",
        "fillin":"600"
      },
      width: 
      {
        "fillin":"378px",
        "btn":"336px",
        "page":"2000px"
      },
      height:
      {
        "fillin":"43px",
        "btn":"66px",
        "page": "60px"
      },
      colors:
      {
        "fillin":"white",
        "fillinText": "black",
        "btn":"#124E86",
        "btnTxt": "white",
      },
      backgroundImage:{
        "logSignup":"url('/src/lib/assets/pylonBack.png')"
      }
    },
  },
  plugins: [
    require("daisyui")
  ],
  
}