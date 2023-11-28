<template>
  <l-header :theme="displayMode" />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useSettingStore } from '@/store';

import LHeader from './Header.vue';

const settingStore = useSettingStore();
const { displayMode } = storeToRefs(settingStore);
</script>
