<template>
  <div class="cover">
    <t-image :src="cover" shape="round" fit="cover" position="center" overlay-trigger="hover" @click="handleClick">
      <template #overlay-content>
        <div class="cover-overlay">
          <calendar-edit-icon size="3em" />
        </div>
      </template>
    </t-image>
    <t-tooltip :content="bangumi.title" placement="mouse">
      <h3 class="bangumi-title">
        <span />
        {{ bangumi.title }}
      </h3>
    </t-tooltip>
    <t-space size="small">
      <t-tag theme="primary">TV</t-tag>
      <t-tag v-if="bangumi.enable" theme="success">{{ $t('components.isSetup.on') }}</t-tag>
      <t-tag v-else theme="danger">{{ $t('components.isSetup.off') }}</t-tag>
    </t-space>
  </div>
</template>
<script setup lang="ts">
import { CalendarEditIcon } from 'tdesign-icons-vue-next';
import { ref } from 'vue';

const bangumi = {
  title: '堤亚穆帝国物语～从断头台开始，公主重生后的逆转人生～',
  enable: true,
};
const cover = ref('https://image.tmdb.org/t/p/original/9ve6LW4ltMMpp9WjoR4MGViOp9V.jpg');
const emit = defineEmits(['edit-bangumi']);

const handleClick = () => {
  emit('edit-bangumi');
};
</script>
<style lang="less" scoped>
.cover {
  width: 150px;

  :deep(.t-image__wrapper) {
    width: 150px;
    height: 225px;
  }

  .cover-overlay {
    background: rgba(0, 0, 0, 0.4);
    color: #fff;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :deep(.t-card__footer) {
    padding: 0;
  }

  :deep(.t-card__footer-wrapper) {
    width: 100%;
  }

  .bangumi-title {
    padding-top: var(--td-comp-paddingTB-s);
    padding-bottom: var(--td-comp-paddingTB-s);
    width: 100%;
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;

    &::after {
      content: '';
      display: block;
    }
  }
}
</style>
