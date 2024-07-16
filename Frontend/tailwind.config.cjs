module.exports = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      fontSize: {
        heading1: "50px",
        heading2: "33px",
        fillin: "27px",
        btnTxt: "28px",
      },
      fontWeight: {
        heading1: "normal",
        heading2: "normal",
        fillin: "600",
      },
      width: {
        fillin: "378px",
        btn: "336px",
        page: "2000px",
      },
      height: {
        fillin: "43px",
        btn: "66px",
        page: "60px",
      },
      colors: {
        fillin: "white",
        fillinText: "black",
        btn: "#124E86",
        btnTxt: "white",
      },
      backgroundImage: {
        logSignup: "url('/src/lib/assets/pylonBack.png')",
      },
      screens: {
        xs: "480px",
        ss: "640px",
        sm: "768px",
        md: "1060px",
        lg: "1200px",
        xl: "1700px",
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: false, // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "dark", // name of one of the included themes for dark mode
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
};
