const theme = require('tailwindcss/defaultTheme')

module.exports = {
  theme: {
    container: {
      center: true,
    },
    extend: {
      screens: {
        'xxl': '1440px',
      },
      colors: {
        'divide-color': theme.colors.gray[800],
      },
      fontSize: {
        'base': '0.925rem',
      },
      backgroundColor: {
        'body': '#444',
        'navbar': theme.colors.gray[800],
        // 'sidebar': '#202020',//theme.colors.gray[900],
        'sidebar': theme.colors.gray[900],
        // 'view': '#202020',//theme.colors.gray[900],
        'view': theme.colors.gray[900],
        'box': theme.colors.gray[800],
      },
      textColor: {
        'highlight': theme.colors.gray[300],
        'primary': theme.colors.gray[400],
        'muted': theme.colors.gray[500],
        'secondary': '#ffed4a',
        'danger': '#e3342f',
      },
      borderColor: {
        'box': theme.colors.gray[700],
      },
    }
  },
  variants: {},
  plugins: [],
}
