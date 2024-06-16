---
title: String
titleTemplate: Commands
description: Essential information to help you get set up with Tachiyomi.
---

# String

Essential information to help you get set up with Rudis.

<script setup>
      import { NGrid, NGi} from 'naive-ui'
      import CommandsCard from '@theme/components/CommandsCard.vue'
</script>


<NGrid :x-gap="24" :y-gap="24" :cols="2">
    <NGi>
        <CommandsCard 
            title="SET"
            description="The Rudis SET command is used to set the value of a given key. If the key has already stored other values, SET will overwrite the old value and ignore the type."
        />
    </NGi>
    <NGi>
        <CommandsCard 
            title="GET"
            description="Get the value of. If the key does not exist the special value is returned. An error is returned if the value stored at is not a string, because only handles string values."
        />
    </NGi>
</NGrid>