import staticAdapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/kit/vite";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [vitePreprocess()],
  kit: {
    adapter: staticAdapter(),
  },
  scss: {
    prependData: `@import './src/style/app.scss';`
  }
};

export default config;
