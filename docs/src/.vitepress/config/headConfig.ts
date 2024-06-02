import type { HeadConfig } from 'vitepress'

const headConfig: HeadConfig[] = [
  ['meta', { name: 'darkreader-lock' }],
  ['meta', { name: 'theme-color', content: '#818CF8' }],
  ['meta', { name: 'msapplication-TileColor', content: '#818CF8' }],
  ['meta', { name: 'viewport', content: 'width=device-width, initial-scale=1.0' }],
  ['meta', { name: 'referrer', content: 'no-referrer-when-downgrade' }],
  ['link', { rel: 'icon', type: 'image/x-icon', href: '/imgs/favicon.ico' }],
]

export default headConfig
