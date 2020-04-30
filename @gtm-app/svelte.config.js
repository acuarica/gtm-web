const {
  preprocess: makeTsPreprocess,
  createEnv,
  readConfigFile,
} = require("@pyoner/svelte-ts-preprocess");

const env = createEnv();
const compilerOptions = readConfigFile(env);
const preprocessOptions = {
  env,
  compilerOptions: {
    ...compilerOptions,
    allowNonTsExtensions: true,
  },
};
const preprocess = makeTsPreprocess(preprocessOptions);

module.exports = {
  dev: process.env.NODE_ENV !== "development",
  preprocess,


  // emitCss: true,

  // Extract CSS into a separate file (recommended).
  // See note below
  css: function (css) {
    // console.log(css.code); // the concatenated CSS
    // console.log(css.map); // a sourcemap

    // creates `main.css` and `main.css.map` — pass `false`
    // as the second argument if you don't want the sourcemap
    css.write('dist/main32.css');
  },
};