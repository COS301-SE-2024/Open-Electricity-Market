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
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        customDarkTheme: {
          primary: "#001f54",
          secondary: "#1282a2",
          accent: "#fefcfb",
          neutral: "#001f5f",
          "base-100": "#0a1128",
          info: "#0000ff",
          success: "#00ff30",
          warning: "#f0ff00",
          error: "#ff0000",
        },
      },
      {
        customLightTheme: {
          primary: "#001f54",
          secondary: "#1282a2",
          accent: "#f2542d",
          neutral: "#001f5f",
          "base-100": "#fefcfb",
          info: "#0000ff",
          success: "#00ff30",
          warning: "#f0ff00",
          error: "#ff0000",
        },
      },
    ], // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "customDarkTheme", // name of one of the included themes for dark mode
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
};
