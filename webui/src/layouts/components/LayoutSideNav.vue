<template>
  <l-side-nav :menu="sideMenu" :theme="displayMode" :is-compact="isSidebarCompact" />
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

import { usePermissionStore, useSettingStore } from '@/store';
import type { MenuRoute } from '@/types/interface';

import LSideNav from './SideNav.vue';

const permissionStore = usePermissionStore();
const settingStore = useSettingStore();
const { routers: menuRouters } = storeToRefs(permissionStore);
const { displayMode, isSidebarCompact } = storeToRefs(settingStore);

const sideMenu = computed(() => {
  return menuRouters.value as Array<MenuRoute>;
});
</script>
