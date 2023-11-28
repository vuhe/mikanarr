<template>
  <div class="result-success">
    <error-circle-icon class="result-success-icon" />
    <div class="result-success-title">{{ $t('pages.bangumi.empty.title') }}</div>
    <div class="result-success-describe">{{ $t('pages.bangumi.empty.subtitle') }}</div>
  </div>
</template>
<script setup lang="ts">
import { ErrorCircleIcon } from 'tdesign-icons-vue-next';
</script>
<style lang="less" scoped>
.result-success {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 75vh;

  &-icon {
    font-size: var(--td-comp-size-xxxxl);
    color: var(--td-text-color-secondary);
  }

  &-title {
    margin-top: var(--td-comp-margin-xxl);
    font: var(--td-font-title-large);
    color: var(--td-text-color-primary);
    text-align: center;
  }

  &-describe {
    margin: var(--td-comp-margin-s) 0 var(--td-comp-margin-xxxl);
    font: var(--td-font-body-medium);
    color: var(--td-text-color-secondary);
  }
}
</style>
