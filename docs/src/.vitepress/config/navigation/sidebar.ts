import type { DefaultTheme } from 'vitepress'

const sidebar: DefaultTheme.SidebarMulti = {
   '/docs/': defaultSidebar(),
}

function defaultSidebar(): DefaultTheme.SidebarItem[] {
  return [
    {
      text: 'Guides',
      items: [
        {
          text: 'Introduce',
          link: '/docs/guides/introduce',
        },
        {
          text: 'Install',
          link: '/docs/guides/install',
        },
        {
          text: 'Configuration',
          link: '/docs/guides/configuration',
        },
      ],
    },
    {
      text: 'Command',
      items: [
        {
          text: 'Key',
          link: '/docs/command/key',
        },
        {
          text: 'String',
          link: '/docs/command/string',
        },
        {
          text: 'Hash',
          link: '/docs/command/hash',
        },
        {
          text: 'Sorted Set',
          link: '/docs/command/sortedSet',
        },
        {
          text: 'Set',
          link: '/docs/command/set',
        },
      ],
    }, 
    {
      text: 'Advanced',
      items: [
        {
          text: 'Secure',
          link: '/docs/advanced/secure',
        },
        {
          text: 'Persistence',
          link: '/docs/advanced/persistence',
        },
      ],
    },
  ]
}

export default sidebar
