<template>
  <div>
    <t-layout key="side" class="t-layout--with-sider">
      <t-aside><layout-side-nav /></t-aside>
      <t-layout>
        <t-header><layout-header /></t-header>
        <t-content><layout-content /></t-content>
      </t-layout>
    </t-layout>
  </div>
</template>

<script setup lang="ts">
import '@/style/layout.less';

import { watch } from 'vue';
import { useRoute } from 'vue-router';

import { prefix } from '@/config/global';

import LayoutContent from './components/LayoutContent.vue';
import LayoutHeader from './components/LayoutHeader.vue';
import LayoutSideNav from './components/LayoutSideNav.vue';

const route = useRoute();

watch(
  () => route.path,
  () => {
    document.querySelector(`.${prefix}-layout`).scrollTo({ top: 0, behavior: 'smooth' });
  },
);
</script>

<style lang="less" scoped></style>
