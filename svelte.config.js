import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import("@sveltejs/kit").Config} */
export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  kit : {
    alias : {
      "$" : "src/",
      "$components" : "src/components",
      "$store" : "src/store.ts",
      "$lib" : "src/lib",
    }
  }
};
