import adapter from "@sveltejs/adapter-static"
import { vitePreprocess } from "@sveltejs/kit/vite"

/** @type {import('@sveltejs/kit').Config} */
export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      pages: "src-tauri/target/frontend-build",
      assets: "src-tauri/target/frontend-build",
      precompress: true,
      fallback: "index.html"
    })
  }
}
