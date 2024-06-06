<script setup lang="ts">
/// <reference types="@types/gtag.js" />

import { computed, onMounted, ref } from 'vue'


const isAndroid = ref(true)

onMounted(() => {
  isAndroid.value = !!navigator.userAgent.match(/android/i)
})
</script>

<template>
  <div>
    <div v-if="!isAndroid" class="custom-block danger">
      <p class="custom-block-title">
        Unsupported operating system
      </p>
      <p>
        <strong>Tachiyomi</strong> is an <strong>Android app</strong> only.
        Use an <strong>Android device</strong> to download and install the app.
      </p>
    </div>
    <div v-if="!isAndroid" class="custom-block warning">
      <p class="custom-block-title">
        Caution
      </p>
      <p>
        Any app for any operating systems other than Android called
        <strong>Tachiyomi</strong> is not affiliated with this project.
      </p>
      <blockquote>
        For more information, read the
        <a href="/docs/faq/general">General FAQ</a>.
      </blockquote>
    </div>
    <span class="version-disclaimer">
      Requires <strong>Android 6.0</strong> or higher.
    </span>
  </div>
</template>

<style lang="stylus">
.download-buttons {
  display: flex
  gap: 0.75em
  justify-content: center
  align-items: center
  margin: 0.75em auto
}

.download-button {
  display: inline-block
  border: 1px solid transparent
  text-align: center
  font-weight: 600
  white-space: nowrap
  transition: color 0.25s, border-color 0.25s, background-color 0.25s
  cursor: pointer
  transition: all 0.3s ease
  border-radius: 20px
  padding: 0 20px
  line-height: 38px
  font-size: 14px

  &:hover {
    text-decoration: none !important
  }

  &.primary {
    border-color: var(--vp-button-brand-border)
    color: var(--vp-button-brand-text)
    background-color: var(--vp-button-brand-bg)

    &:hover {
      border-color: var(--vp-button-brand-hover-border)
      color: var(--vp-button-brand-hover-text)
      background-color: var(--vp-button-brand-hover-bg)
    }

    &:active {
      border-color: var(--vp-button-brand-active-border)
      color: var(--vp-button-brand-active-text)
      background-color: var(--vp-button-brand-active-bg)
    }
  }

  &.secondary {
    border-color: var(--vp-button-alt-border)
    color: var(--vp-button-alt-text)
    background-color: var(--vp-button-alt-bg)

    &:hover {
      border-color: var(--vp-button-alt-hover-border)
      color: var(--vp-button-alt-hover-text)
      background-color: var(--vp-button-alt-hover-bg)
    }

    &:active {
      border-color: var(--vp-button-alt-active-border)
      color: var(--vp-button-alt-active-text)
      background-color: var(--vp-button-alt-active-bg)
    }
  }

  svg {
    display: inline-block
    vertical-align: middle
    margin-right: 0.5em
    font-size: 1.25em
  }

  .text {
    margin-right: 10px
  }

  .version {
    font-size: 0.8em
  }
}

.version-disclaimer {
  display: block
  text-align: center
  margin: 0.75em auto
  font-size: 0.75rem
}
</style>
