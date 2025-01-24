module.exports = {
  mode: "all", // This is not a valid option in Tailwind CSS 4.0.0. Remove it.
  content: [
    // Include all Rust, HTML, and CSS files in the src directory
    "./src/**/*.{rs,html,css}",
    // Include all HTML files in the output (dist) directory
    "./dist/**/*.html",
  ],
  theme: {
    extend: {}, // Extend the default theme if needed
  },
  plugins: [], // Add any Tailwind plugins here
};