/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{rs,html}"],
  theme: {
    extend: {
      colors: {
        primary: "#0f172a",
        "ct-dark-600": "#222",
        "ct-dark-200": "#e5e7eb",
        "ct-dark-100": "#f5f6f7",
        "ct-blue-600": "#2363eb",
        "ct-yellow-600": "#f9d13e",
        "ct-red-500": "#ef4444",
      },
      fontFamily: {
        // Poppins: ["Poppins, sans-serif"],
        Kanit: ["Kanit, sans-serif"],
      },
    },
  },
  plugins: [],
};