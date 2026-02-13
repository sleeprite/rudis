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
              text: 'EXISTS',
              link: '/zh/docs/commands/key/exists',
            },
            {
              text: 'EXPIRE',
              link: '/zh/docs/commands/key/expire',
            },
            {
              text: 'EXPIREAT',
              link: '/zh/docs/commands/key/expireat',
            },
            {
              text: 'KEYS',
              link: '/zh/docs/commands/key/keys',
            },
            {
              text: 'MOVE',
              link: '/zh/docs/commands/key/move',
            },
            {
              text: 'PERSIST',
              link: '/zh/docs/commands/key/persist',
            },
            {
              text: 'PEXPIRE',
              link: '/zh/docs/commands/key/pexpire',
            },
            {
              text: 'PEXPIREAT',
              link: '/zh/docs/commands/key/pexpireat',
            },
            {
              text: 'PTTL',
              link: '/zh/docs/commands/key/pttl',
            },
            {
              text: 'RANDOMKEY',
              link: '/zh/docs/commands/key/randomkey',
            },
            {
              text: 'RENAME',
              link: '/zh/docs/commands/key/rename',
            },
            {
              text: 'RENAMENX',
              link: '/zh/docs/commands/key/renamenx',
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
              text: 'APPEND',
              link: '/zh/docs/commands/string/append',
            },
            {
              text: 'DECR',
              link: '/zh/docs/commands/string/decr',
            },
            {
              text: 'DECRBY',
              link: '/zh/docs/commands/string/decrby',
            },
            {
              text: 'GET',
              link: '/zh/docs/commands/string/get',
            },
            {
              text: 'GETRANGE',
              link: '/zh/docs/commands/string/getrange',
            },
            {
              text: 'GETSET',
              link: '/zh/docs/commands/string/getset',
            },
            {
              text: 'INCR',
              link: '/zh/docs/commands/string/incr',
            },
            {
              text: 'INCRBY',
              link: '/zh/docs/commands/string/incrby',
            },
            {
              text: 'INCRBYFLOAT',
              link: '/zh/docs/commands/string/incrbyfloat',
            },
            {
              text: 'MGET',
              link: '/zh/docs/commands/string/mget',
            },
            {
              text: 'MSET',
              link: '/zh/docs/commands/string/mset',
            },
            {
              text: 'SET',
              link: '/zh/docs/commands/string/set',
            },
            {
              text: 'STRLEN',
              link: '/zh/docs/commands/string/strlen',
            },
          ]
        },
        {
          text: '列表',
          link: '/zh/docs/commands/list',
          collapsed: true,
          items: [
            {
              text: 'LINDEX',
              link: '/zh/docs/commands/list/lindex',
            },
            {
              text: 'LLEN',
              link: '/zh/docs/commands/list/llen',
            },
            {
              text: 'LPOP',
              link: '/zh/docs/commands/list/lpop',
            },
            {
              text: 'LPUSH',
              link: '/zh/docs/commands/list/lpush',
            },
            {
              text: 'LPUSHX',
              link: '/zh/docs/commands/list/lpushx',
            },
            {
              text: 'LRANGE',
              link: '/zh/docs/commands/list/lrange',
            },
            {
              text: 'LSET',
              link: '/zh/docs/commands/list/lset',
            },
            {
              text: 'RPUSH',
              link: '/zh/docs/commands/list/rpush',
            },
            {
              text: 'RPUSHX',
              link: '/zh/docs/commands/list/rpushx',
            },
            {
              text: 'RPOP',
              link: '/zh/docs/commands/list/rpop',
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
              text: 'HEXISTS',
              link: '/zh/docs/commands/hash/hexists',
            },
            {
              text: 'HGET',
              link: '/zh/docs/commands/hash/hget',
            },
            {
              text: 'HGETALL',
              link: '/zh/docs/commands/hash/hgetall',
            },
            {
              text: 'HKEYS',
              link: '/zh/docs/commands/hash/hkeys',
            },
            {
              text: 'HLEN',
              link: '/zh/docs/commands/hash/hlen',
            },
            {
              text: 'HMGET',
              link: '/zh/docs/commands/hash/hmget',
            },
            {
              text: 'HMSET',
              link: '/zh/docs/commands/hash/hmset',
            },
            {
              text: 'HSET',
              link: '/zh/docs/commands/hash/hset',
            },
            {
              text: 'HSETNX',
              link: '/zh/docs/commands/hash/hsetnx',
            },
            {
              text: 'HSTRLEN',
              link: '/zh/docs/commands/hash/hstrlen',
            },
            {
              text: 'HVALS',
              link: '/zh/docs/commands/hash/hvals',
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
              text: 'SCARD',
              link: '/zh/docs/commands/set/scard',
            },
            {
              text: 'SINTER',
              link: '/zh/docs/commands/set/sinter',
            },
            {
              text: 'SISMEMBER',
              link: '/zh/docs/commands/set/sismember',
            },
            {
              text: 'SMEMBERS',
              link: '/zh/docs/commands/set/smembers',
            },
            {
              text: 'SPOP',
              link: '/zh/docs/commands/set/spop',
            },
            {
              text: 'SREM',
              link: '/zh/docs/commands/set/srem',
            },
            {
              text: 'SUNION',
              link: '/zh/docs/commands/set/sunion',
            },
            {
              text: 'SUNIONSTORE',
              link: '/zh/docs/commands/set/sunionstore',
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
            {
              text: 'ZCARD',
              link: '/zh/docs/commands/sortedSet/zcard',
            },
            {
              text: 'ZCOUNT',
              link: '/zh/docs/commands/sortedSet/zcount',
            },
            {
              text: 'ZRANK',
              link: '/zh/docs/commands/sortedSet/zrank',
            },
            {
              text: 'ZREM',
              link: '/zh/docs/commands/sortedSet/zrem',
            },
            {
              text: 'ZSCORE',
              link: '/zh/docs/commands/sortedSet/zscore',
            },
          ]
        },
        {
          text: 'HyperLogLog',
          link: '/zh/docs/commands/hyperloglog',
          collapsed: true,
          items: [
            {
              text: 'PFADD',
              link: '/zh/docs/commands/hyperloglog/pfadd',
            },
            {
              text: 'PFCOUNT',
              link: '/zh/docs/commands/hyperloglog/pfcount',
            },
            {
              text: 'PFMERGE',
              link: '/zh/docs/commands/hyperloglog/pfmerge',
            },
          ]
        },
        {
          text: 'Geo',
          link: '/zh/docs/commands/geo',
          collapsed: true,
          items: [
            {
              text: 'GEOADD',
              link: '/zh/docs/commands/geo/geoadd',
            },
            {
              text: 'GEOPOS',
              link: '/zh/docs/commands/geo/geopos',
            },
            {
              text: 'GEODIST',
              link: '/zh/docs/commands/geo/geodist',
            },
            {
              text: 'GEORADIUS',
              link: '/zh/docs/commands/geo/georadius',
            },
            {
              text: 'GEORADIUSBYMEMBER',
              link: '/zh/docs/commands/geo/georadiusbymember',
            },
            {
              text: 'GEOHASH',
              link: '/zh/docs/commands/geo/geohash',
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
            {
              text: 'CLIENT',
              link: '/zh/docs/commands/generic/client',
            },
            {
              text: 'ECHO',
              link: '/zh/docs/commands/generic/echo',
            },
            {
              text: 'PING',
              link: '/zh/docs/commands/generic/ping',
            },
            {
              text: 'SELECT',
              link: '/zh/docs/commands/generic/select',
            },
          ]
        },
      ],
    },
    {
      text: '应用',
      items: [
        {
          text: '在 Rust 中使用',
          link: '/zh/docs/guides/rust',
        },
        {
          text: '在 Java 中使用',
          link: '/zh/docs/guides/java',
        },
      ],
    },
    {
      text: '高级',
      items: [
        {
          text: '安全',
          link: '/zh/docs/advance/security',
        },
        {
          text: '事务',
          link: '/zh/docs/advance/transactions',
        },
        {
          text: '备份恢复',
          link: '/zh/docs/advance/persistence',
        },
        {
          text: '主从同步',
          link: '/zh/docs/advance/replication',
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
              text: 'EXISTS',
              link: '/docs/commands/key/exists',
            },
            {
              text: 'EXPIRE',
              link: '/docs/commands/key/expire',
            },
            {
              text: 'EXPIREAT',
              link: '/docs/commands/key/expireat',
            },
            {
              text: 'KEYS',
              link: '/docs/commands/key/keys',
            },
            {
              text: 'MOVE',
              link: '/docs/commands/key/move',
            },
            {
              text: 'PERSIST',
              link: '/docs/commands/key/persist',
            },
            {
              text: 'PEXPIRE',
              link: '/docs/commands/key/pexpire',
            },
            {
              text: 'PEXPIREAT',
              link: '/docs/commands/key/pexpireat',
            },
            {
              text: 'PTTL',
              link: '/docs/commands/key/pttl',
            },
            {
              text: 'RANDOMKEY',
              link: '/docs/commands/key/randomkey',
            },
            {
              text: 'RENAME',
              link: '/docs/commands/key/rename',
            },
            {
              text: 'RENAMENX',
              link: '/docs/commands/key/renamenx',
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
              text: 'APPEND',
              link: '/docs/commands/string/append',
            },
            {
              text: 'DECR',
              link: '/docs/commands/string/decr',
            },
            {
              text: 'DECRBY',
              link: '/docs/commands/string/decrby',
            },
            {
              text: 'GET',
              link: '/docs/commands/string/get',
            },
            {
              text: 'GETRANGE',
              link: '/docs/commands/string/getrange',
            },
            {
              text: 'GETSET',
              link: '/docs/commands/string/getset',
            },
            {
              text: 'INCR',
              link: '/docs/commands/string/incr',
            },
            {
              text: 'INCRBY',
              link: '/docs/commands/string/incrby',
            },
            {
              text: 'INCRBYFLOAT',
              link: '/docs/commands/string/incrbyfloat',
            },
            {
              text: 'MGET',
              link: '/docs/commands/string/mget',
            },
            {
              text: 'MSET',
              link: '/docs/commands/string/mset',
            },
            {
              text: 'SET',
              link: '/docs/commands/string/set',
            },
            {
              text: 'STRLEN',
              link: '/docs/commands/string/strlen',
            },
          ]
        },
        {
          text: 'List',
          link: '/docs/commands/list',
          collapsed: true,
          items: [
            {
              text: 'LINDEX',
              link: '/docs/commands/list/lindex',
            },
            {
              text: 'LLEN',
              link: '/docs/commands/list/llen',
            },
            {
              text: 'LPOP',
              link: '/docs/commands/list/lpop',
            },
            {
              text: 'LPUSH',
              link: '/docs/commands/list/lpush',
            },
            {
              text: 'LPUSHX',
              link: '/docs/commands/list/lpushx',
            },
            {
              text: 'LRANGE',
              link: '/docs/commands/list/lrange',
            },
            {
              text: 'LSET',
              link: '/docs/commands/list/lset',
            },
            {
              text: 'RPUSH',
              link: '/docs/commands/list/rpush',
            },
            {
              text: 'RPUSHX',
              link: '/docs/commands/list/rpushx',
            },
            {
              text: 'RPOP',
              link: '/docs/commands/list/rpop',
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
              text: 'HEXISTS',
              link: '/docs/commands/hash/hexists',
            },
            {
              text: 'HGET',
              link: '/docs/commands/hash/hget',
            },
            {
              text: 'HGETALL',
              link: '/docs/commands/hash/hgetall',
            },
            {
              text: 'HKEYS',
              link: '/docs/commands/hash/hkeys',
            },
            {
              text: 'HLEN',
              link: '/docs/commands/hash/hlen',
            },
            {
              text: 'HMGET',
              link: '/docs/commands/hash/hmget',
            },
            {
              text: 'HMSET',
              link: '/docs/commands/hash/hmset',
            },
            {
              text: 'HSET',
              link: '/docs/commands/hash/hset',
            },
            {
              text: 'HSETNX',
              link: '/docs/commands/hash/hsetnx',
            },
            {
              text: 'HSTRLEN',
              link: '/docs/commands/hash/hstrlen',
            },
            {
              text: 'HVALS',
              link: '/docs/commands/hash/hvals',
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
              text: 'SCARD',
              link: '/docs/commands/set/scard',
            },
            {
              text: 'SINTER',
              link: '/docs/commands/set/sinter',
            },
            {
              text: 'SISMEMBER',
              link: '/docs/commands/set/sismember',
            },
            {
              text: 'SMEMBERS',
              link: '/docs/commands/set/smembers',
            },
            {
              text: 'SPOP',
              link: '/docs/commands/set/spop',
            },
            {
              text: 'SREM',
              link: '/docs/commands/set/srem',
            },
            {
              text: 'SUNION',
              link: '/docs/commands/set/sunion',
            },
            {
              text: 'SUNIONSTORE',
              link: '/docs/commands/set/sunionstore',
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
            {
              text: 'ZCARD',
              link: '/docs/commands/sortedSet/zcard',
            },
            {
              text: 'ZCOUNT',
              link: '/docs/commands/sortedSet/zcount',
            },
            {
              text: 'ZRANK',
              link: '/docs/commands/sortedSet/zrank',
            },
            {
              text: 'ZREM',
              link: '/docs/commands/sortedSet/zrem',
            },
            {
              text: 'ZSCORE',
              link: '/docs/commands/sortedSet/zscore',
            },
          ]
        },
        {
          text: 'HyperLogLog',
          link: '/docs/commands/hyperloglog',
          collapsed: true,
          items: [
            {
              text: 'PFADD',
              link: '/docs/commands/hyperloglog/pfadd',
            },
            {
              text: 'PFCOUNT',
              link: '/docs/commands/hyperloglog/pfcount',
            },
            {
              text: 'PFMERGE',
              link: '/docs/commands/hyperloglog/pfmerge',
            },
          ]
        },
        {
          text: 'Geo',
          link: '/docs/commands/geo',
          collapsed: true,
          items: [
            {
              text: 'GEOADD',
              link: '/docs/commands/geo/geoadd',
            },
            {
              text: 'GEOPOS',
              link: '/docs/commands/geo/geopos',
            },
            {
              text: 'GEODIST',
              link: '/docs/commands/geo/geodist',
            },
            {
              text: 'GEORADIUS',
              link: '/docs/commands/geo/georadius',
            },
            {
              text: 'GEORADIUSBYMEMBER',
              link: '/docs/commands/geo/georadiusbymember',
            },
            {
              text: 'GEOHASH',
              link: '/docs/commands/geo/geohash',
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
            {
              text: 'CLIENT',
              link: '/docs/commands/generic/client',
            },
            {
              text: 'ECHO',
              link: '/docs/commands/generic/echo',
            },
            {
              text: 'PING',
              link: '/docs/commands/generic/ping',
            },
            {
              text: 'SELECT',
              link: '/docs/commands/generic/select',
            },
          ]
        },
      ],
    },
    {
      text: 'Application Guides',
      items: [
        {
          text: 'Using Rudis in Rust',
          link: '/docs/guides/rust',
        },
        {
          text: 'Using Rudis in Java',
          link: '/docs/guides/java',
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
        {
          text: 'Transactions',
          link: '/docs/advance/transactions',
        },
        {
          text: 'Replication',
          link: '/docs/advance/replication',
        },
      ],
    }
  ]
}

export default sidebar