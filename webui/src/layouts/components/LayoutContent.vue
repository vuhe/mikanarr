<template>
  <t-layout :class="`${prefix}-layout`">
    <t-content :class="`${prefix}-content-layout`">
      <l-content />
    </t-content>
    <t-footer :class="`${prefix}-footer-layout`">
      <l-footer />
    </t-footer>
  </t-layout>
</template>

<script setup lang="ts">
import { prefix } from '@/config/global';

import LContent from './Content.vue';
import LFooter from './Footer.vue';
</script>
