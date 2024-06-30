import process from 'node:process'
import { URL, fileURLToPath } from 'node:url'
import { defineConfig, loadEnv } from 'vitepress'
import ElementPlus from 'unplugin-element-plus/vite'

import markdownConfig from './config/markdownConfig'

// For use with loading Markdown plugins
import themeConfig from './config/themeConfig'

// Theme related config
import headConfig from './config/headConfig'

// Provides how to generate Meta head tag

const title = 'Rudis'
const description = 'Discover and read manga, webtoons, comics, and more – easier than ever on your Android device.'

const env = loadEnv('', process.cwd())
const hostname: string = env.VITE_HOSTNAME || 'http://localhost:4173'

export default defineConfig({
  outDir: '../dist',
  title,
  lang: "en",
  description,
  locales: {
    root: {
      label: 'English',
      lang: 'en',
    },
    zh: {
      label: '简体中文',
      lang: 'zh',
      link: "/zh/",
      themeConfig: {
        nav: [{
          text: 'Docs',
          link: '/zh/docs/guides/introduce',
          activeMatch: '/zh/docs/',
        }],
      }
    },
  },
  sitemap: {
    hostname,
  },
  cleanUrls: true,
  lastUpdated: true,
  head: headConfig,
  markdown: markdownConfig,
  themeConfig,
  vite: {
    resolve: {
      alias: [
        {
          find: /^.*VPSwitchAppearance\.vue$/,
          replacement: fileURLToPath(
            new URL('./theme/components/CustomSwitchAppearance.vue', import.meta.url),
          ),
        },
      ],
    },
    plugins: [ElementPlus({})],
    ssr: {
      noExternal: ['element-plus'],
    },
  },
})