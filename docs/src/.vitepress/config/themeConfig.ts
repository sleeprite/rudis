import type { DefaultTheme } from 'vitepress'

import nav from './navigation/navbar'
import sidebar from './navigation/sidebar'

const themeConfig: DefaultTheme.Config = {
  logo: {
    src: '/imgs/logo.png',
    height: 24,
    width: 24,
  },

  nav,
  sidebar,

  outline: [2, 3],

  socialLinks: [
    {
      icon: 'github',
      link: 'https://github.com/tachiyomiorg/tachiyomi',
      ariaLabel: 'Project GitHub',
    },
    {
      icon: 'facebook',
      link: 'https://facebook.com/tachiyomiorg',
      ariaLabel: 'Facebook Page',
    },
  ],

  editLink: {
    pattern: 'https://github.com/tachiyomiorg/website/edit/main/website/src/:path',
    text: 'Help us improve this page',
  },

  lastUpdated: {
    text: 'Last updated',
    formatOptions: {
      forceLocale: true,
      dateStyle: 'long',
      timeStyle: 'short',
    },
  },

  search: {
    provider: 'algolia',
    options: {
      appId: '2C8EHFTRW7',
      apiKey: 'ee38c6e04295e4d206399ab59a58ea9a',
      indexName: 'tachiyomi',
    },
  },
}

export default themeConfig
