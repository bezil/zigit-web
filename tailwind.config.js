module.exports = {
    mode: "jit",
    content: {
      files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
      extend: {
        colors: {
          'zig-grey': '#404040',
          'zig-orange': '#ffb152',
        },
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
  };
