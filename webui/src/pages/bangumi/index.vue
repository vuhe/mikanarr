<template>
  <div>
    <div class="list-card-operation">
      <t-button theme="danger" @click="formDialogVisible = true"> {{ $t('pages.bangumi.truncate') }} </t-button>
      <div class="search-input">
        <t-input v-model="searchValue" :placeholder="$t('pages.bangumi.search')" clearable @change="handleSearch">
          <template #suffix-icon>
            <search-icon v-if="searchValue === ''" size="var(--td-comp-size-xxxs)" />
          </template>
        </t-input>
      </div>
    </div>

    <dialog-form v-model:visible="formDialogVisible" :data="formData" />

    <template v-if="!dataLoading">
      <t-row :gutter="[16, 32]">
        <t-col v-for="bangumi in [...Array(8).keys()]" :key="bangumi">
          <bangumi-cover @edit-bangumi="handleEditBangumi" />
        </t-col>
      </t-row>
    </template>

    <div v-else-if="dataLoading" class="list-card-loading">
      <t-loading size="large" :text="$t('pages.bangumi.loading')" />
    </div>

    <div v-else>
      <bangumi-empty />
    </div>
  </div>
</template>

<script setup lang="ts">
import { SearchIcon } from 'tdesign-icons-vue-next';
import { onMounted, ref } from 'vue';

import { getBangumiList } from '@/api/bangumi';
import type { CardProductType } from '@/components/product-card/index.vue';

import BangumiCover from './BangumiCover.vue';
import BangumiEmpty from './BangumiEmpty.vue';
import type { FormData } from './DialogForm.vue';
import DialogForm from './DialogForm.vue';

const INITIAL_DATA: FormData = {
  name: '',
  status: '',
  description: '',
  type: '0',
  mark: '',
  amount: 0,
};

const bangumiList = ref([]);
const dataLoading = ref(true);

const fetchData = async () => {
  try {
    const { list } = await getBangumiList(searchValue.value);
    bangumiList.value = list;
  } catch (e) {
    console.log(e);
  } finally {
    dataLoading.value = false;
  }
};

onMounted(() => {
  fetchData();
});

const formDialogVisible = ref(false);
const searchValue = ref('');
const formData = ref({ ...INITIAL_DATA });

const handleSearch = () => {
  fetchData();
};
const handleEditBangumi = (product: CardProductType) => {
  formData.value = {
    name: product.name,
    status: product?.isSetup ? '1' : '0',
    description: product.description,
    type: product.type.toString(),
    mark: '',
    amount: 0,
  };
  formDialogVisible.value = true;
};
</script>

<style lang="less" scoped>
.list-card {
  height: 100%;

  &-operation {
    display: flex;
    justify-content: space-between;
    margin-bottom: var(--td-comp-margin-xxl);

    .search-input {
      width: 360px;
    }
  }

  &-loading {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>
