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
        xs: "375px",
        ss: "500px",
        sm: "768px",
        md: "1060px",
        lg: "1200px",
        xl: "1700px",
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        amplifyLight: {
          primary: "#008cff",
          "primary-content": "#000716",
          secondary: "#004fe5",
          "secondary-content": "#cddffe",
          accent: "#00e5be",
          "accent-content": "#00120d",
          neutral: "#2d2b26",
          "neutral-content": "#d1d0cf",
          "base-100": "#edffff",
          "base-200": "#cedede",
          "base-300": "#b0bebe",
          "base-content": "#141616",
          info: "#009dee",
          "info-content": "#000914",
          success: "#00f278",
          "success-content": "#001405",
          warning: "#f2c800",
          "warning-content": "#140f00",
          error: "#ff6693",
          "error-content": "#160308",
        },
      },
      {
        amplifyDark: {
          primary: "#065ae1",
          "primary-content": "#000a16",
          secondary: "#00b3ff",
          "secondary-content": "#000c16",
          accent: "#d1cfe2",
          "accent-content": "#000716",
          neutral: "#000f16",
          "neutral-content": "#c4c9cb",
          "base-100": "#1d232a", // 1d232a
          "base-200": "#16161a",
          "base-300": "#000000",
          "base-content": "#d1d0cf",
          info: "#00d2ff",
          "info-content": "#001016",
          success: "#00f231",
          "success-content": "#001401",
          warning: "#fb6e00",
          "warning-content": "#150400",
          error: "#ff191e",
          "error-content": "#160000",
        },
      },
    ], // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "amplifyDark", // name of one of the included themes for dark mode
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
};
