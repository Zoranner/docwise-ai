// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  ssr: false,
  modules: ["@nuxt/ui", "@nuxt/eslint"],
  css: ["~/assets/css/main.css"],
  devtools: { enabled: true },
  devServer: {
    port: 3000,
    strictPort: true,
  },
  app: {
    head: {
      title: "Docwise",
      htmlAttrs: { lang: "zh-CN" },
      link: [
        {
          rel: "stylesheet",
          href: "https://fonts.googleapis.com/css2?family=Source+Sans+3:ital,wght@0,400;0,500;0,600;0,700;1,400&display=swap",
        },
      ],
    },
  },
});
