---
title: Key
titleTemplate: Commands
description: Essential information to help you get set up with Tachiyomi.
---

# Key

The Redis key command is used to manage Redis keys.


<script setup>
      import { NGrid, NGi} from 'naive-ui'
      import CommandsCard from '@theme/components/CommandsCard.vue'
</script>


<NGrid :x-gap="24" :y-gap="24" :cols="2">
    <NGi>
        <CommandsCard 
            title="DEL"
            description="Removes the specified keys. A key is ignored if it does not exist."
        />
    </NGi>
    <NGi>
        <CommandsCard 
            title="RENAME"
            description="Renames key to newkey. It returns an error when key does not exist. If newkey already exists it is overwritten, when this happens RENAME executes an implicit DEL operation, so if the deleted key contains a very big value it may cause high latency even if RENAME itself is usually a constant-time operation."
        />
    </NGi>
    <NGi>
        <CommandsCard 
            title="TYPE"
            description="Like TTL this command returns the remaining time to live of a key that has an expire set, with the sole difference that TTL returns the amount of remaining time in seconds while PTTL returns it in milliseconds."
        />
    </NGi>
    <NGi>
        <CommandsCard 
            title="TTL"
            description="Returns the string representation of the type of the value stored at key. The different types that can be returned are: string, list, set, zset and hash."
        />
    </NGi>
</NGrid>