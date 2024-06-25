import type { DefaultTheme } from 'vitepress'

const sidebar: DefaultTheme.SidebarMulti = {
  "/docs/": enSidebar(),
  '/zh/docs/': zhSidebar()
}

function zhSidebar(): DefaultTheme.SidebarItem[] {
  return [
    {
      text: '更新日志',
      link: '/zh/docs/guides/changelog',
    },
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
        {
          text: '协议',
          link: '/zh/docs/guides/protocolSpec',
        },
      ],
    },
    {
      text: '命令',
      items: [
        {
          text: '键',
          link: '/zh/docs/commands/key',
          collapsed: true,
          items: [
            {
              text: 'DEL',
              link: '/zh/docs/commands/key/del',
            },
            {
              text: 'RENAME',
              link: '/zh/docs/commands/key/rename',
            },
            {
              text: 'TTL',
              link: '/zh/docs/commands/key/ttl',
            },
            {
              text: 'TYPE',
              link: '/zh/docs/commands/key/type',
            },
          ]
        },
        {
          text: '字符串',
          link: '/zh/docs/commands/string',
          collapsed: true,
          items: [
            {
              text: 'SET',
              link: '/zh/docs/commands/string/set',
            },
            {
              text: 'GET',
              link: '/zh/docs/commands/string/get',
            },
          ]
        },
        {
          text: '哈希',
          link: '/zh/docs/commands/hash',
          collapsed: true,
          items: [
            {
              text: 'HDEL',
              link: '/zh/docs/commands/hash/hdel',
            },
            {
              text: 'HSET',
              link: '/zh/docs/commands/hash/hset',
            },
          ]
        },
        {
          text: '列表',
          link: '/zh/docs/commands/list',
          collapsed: true,
          items: [
            {
              text: 'LLEN',
              link: '/zh/docs/commands/list/llen',
            },
            {
              text: 'LPOP',
              link: '/zh/docs/commands/list/lpop',
            },
          ]
        },
        {
          text: '集合',
          link: '/zh/docs/commands/set',
          collapsed: true,
          items: [
            {
              text: 'SADD',
              link: '/zh/docs/commands/set/sadd',
            },
            {
              text: 'SPOP',
              link: '/zh/docs/commands/set/spop',
            },
          ]
        },
        {
          text: '有序集合',
          link: '/zh/docs/commands/sortedSet',
          collapsed: true,
          items: [
            {
              text: 'ZADD',
              link: '/zh/docs/commands/sortedSet/zadd',
            },
          ]
        },
        {
          text: '通用',
          link: '/zh/docs/commands/generic',
          collapsed: true,
          items: [
            {
              text: 'AUTH',
              link: '/zh/docs/commands/generic/auth',
            },
          ]
        },
      ],
    },
    {
      text: '高级',
      items: [
        {
          text: '备份与恢复',
          link: '/zh/docs/advance/persistence',
        },
        {
          text: '安全',
          link: '/zh/docs/advance/security',
        },
      ],
    }
  ]
}

function enSidebar(): DefaultTheme.SidebarItem[] {
  return [
    {
      text: 'Changelog',
      link: '/docs/guides/changelog',
    },
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
        {
          text: 'Protocol Spec',
          link: '/docs/guides/protocolSpec',
        },
      ],
    },
    {
      text: 'Command',
      items: [
        {
          text: 'Key',
          link: '/docs/commands/key',
          collapsed: true,
          items: [
            {
              text: 'DEL',
              link: '/docs/commands/key/del',
            },
            {
              text: 'RENAME',
              link: '/docs/commands/key/rename',
            },
            {
              text: 'TTL',
              link: '/docs/commands/key/ttl',
            },
            {
              text: 'TYPE',
              link: '/docs/commands/key/type',
            },
          ]
        },
        {
          text: 'String',
          link: '/docs/commands/string',
          collapsed: true,
          items: [
            {
              text: 'SET',
              link: '/docs/commands/string/set',
            },
            {
              text: 'GET',
              link: '/docs/commands/string/get',
            },
          ]
        },
        {
          text: 'Hash',
          link: '/docs/commands/hash',
          collapsed: true,
          items: [
            {
              text: 'HDEL',
              link: '/docs/commands/hash/hdel',
            },
            {
              text: 'HSET',
              link: '/docs/commands/hash/hset',
            },
          ]
        },
        {
          text: 'List',
          link: '/docs/commands/list',
          collapsed: true,
          items: [
            {
              text: 'LLEN',
              link: '/docs/commands/list/llen',
            },
            {
              text: 'LPOP',
              link: '/docs/commands/list/lpop',
            },
          ]
        },
        {
          text: 'Set',
          link: '/docs/commands/set',
          collapsed: true,
          items: [
            {
              text: 'SADD',
              link: '/docs/commands/set/sadd',
            },
            {
              text: 'SPOP',
              link: '/docs/commands/set/spop',
            },
          ]
        },
        {
          text: 'Sorted set',
          link: '/docs/commands/sortedSet',
          collapsed: true,
          items: [
            {
              text: 'ZADD',
              link: '/docs/commands/sortedSet/zadd',
            },
          ]
        },
        {
          text: 'Generic',
          link: '/docs/commands/generic',
          collapsed: true,
          items: [
            {
              text: 'AUTH',
              link: '/docs/commands/generic/auth',
            },
          ]
        },
      ],
    },
    {
      text: 'Advanced',
      items: [
        {
          text: 'Persistence',
          link: '/docs/advance/persistence',
        },
        {
          text: 'Security',
          link: '/docs/advance/security',
        },
      ],
    }
  ]
}

export default sidebar
