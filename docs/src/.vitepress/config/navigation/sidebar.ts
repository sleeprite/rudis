import type { DefaultTheme } from 'vitepress'

const sidebar: DefaultTheme.SidebarMulti = {
   '/zh/docs/': zhSidebar(),
   "/docs/": enSidebar()
}

function zhSidebar(): DefaultTheme.SidebarItem[] {
  return [
    {
      text: '指南',
      items: [
        {
          text: '介绍',
          link: '/zh/docs/guides/introduce',
        },
        {
          text: '安装',
          link: '/zh/docs/guides/install',
        },
        {
          text: '配置',
          link: '/zh/docs/guides/configuration',
        },
      ],
    },
    {
      text: '命令',
      items: [
        {
          text: '键',
          link: '/zh/docs/commands/key',
        },
        {
          text: '字符串',
          link: '/zh/docs/commands/string',
        },
        {
          text: '集合',
          link: '/zh/docs/commands/set',
        },
        {
          text: '有序集合',
          link: '/zh/docs/commands/sortedSet',
        },
        {
          text: '列表',
          link: '/zh/docs/commands/list',
        },
        {
          text: '哈希',
          link: '/zh/docs/commands/hash',
        },
      ],
    },
    {
      text: '高级',
      items: [
        {
          text: '安全',
          link: '/zh/docs/advance/secure',
        },
        {
          text: '持久化',
          link: '/zh/docs/advance/persistent',
        },
      ],
    }, {
      text: "工具",
      items: [
        {
          text: 'Rust',
          link: '/docs/libraries/rust',
        },
        {
          text: 'PHP',
          link: '/docs/libraries/php',
        },
        {
          text: 'Java',
          link: '/docs/libraries/java',
        },
      ],
    }
  ]
}

function enSidebar(): DefaultTheme.SidebarItem[] {
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
      text: 'Commands',
      items: [
        {
          text: 'Key',
          link: '/docs/commands/key',
        },
        {
          text: 'String',
          link: '/docs/commands/string',
        },
        {
          text: 'Set',
          link: '/docs/commands/set',
        },
        {
          text: 'Sorted set',
          link: '/docs/commands/sortedSet',
        },
        {
          text: 'List',
          link: '/docs/commands/list',
        },
        {
          text: 'Hash',
          link: '/docs/commands/hash',
        },
      ],
    },
    {
      text: 'Advanced',
      items: [
        {
          text: 'Secure',
          link: '/docs/advance/secure',
        },
        {
          text: 'Persistent',
          link: '/docs/advance/persistent',
        },
      ],
    }, {
      text: "libraries",
      items: [
        {
          text: 'Rust',
          link: '/docs/libraries/rust',
        },
        {
          text: 'PHP',
          link: '/docs/libraries/php',
        },
        {
          text: 'Java',
          link: '/docs/libraries/java',
        },
      ],
    }
  ]
}

export default sidebar
