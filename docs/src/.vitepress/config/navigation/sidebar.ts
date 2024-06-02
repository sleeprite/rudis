import type { DefaultTheme } from 'vitepress'

const sidebar: DefaultTheme.SidebarMulti = {
  '/extensions/': defaultSidebar(),
  '/docs/': defaultSidebar(),
  '/changelogs/': defaultSidebar(),
  '/news/': defaultSidebar(),
  '/sandbox/': defaultSidebar(),
}

function defaultSidebar(): DefaultTheme.SidebarItem[] {
  return [
    {
      text: 'Guides',
      items: [
        {
          text: 'Getting started',
          link: '/docs/guides/getting-started',
        },
        {
          text: 'Install',
          link: '/extensions/',
        },
      ],
    },
  ]
}

export default sidebar
