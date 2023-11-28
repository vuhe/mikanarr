<template>
  <div class="list-common-table">
    <t-form ref="form" :data="searchParam" :label-width="80" colon @submit="fetchData" @reset="searchReset">
      <t-row>
        <t-col :span="10">
          <t-row :gutter="[24, 24]">
            <t-col :span="4">
              <t-form-item :label="$t('pages.rss.field.name')" name="name">
                <t-input
                  v-model="searchParam.name"
                  class="form-item-content"
                  type="search"
                  :placeholder="$t('pages.rss.placeholder.name')"
                  :style="{ minWidth: '134px' }"
                />
              </t-form-item>
            </t-col>
            <t-col :span="4">
              <t-form-item :label="$t('pages.rss.field.category')" name="status">
                <t-select
                  v-model="searchParam.category"
                  class="form-item-content"
                  :options="RSS_CATEGORY_OPTIONS"
                  :placeholder="$t('pages.rss.placeholder.category')"
                  clearable
                />
              </t-form-item>
            </t-col>
          </t-row>
        </t-col>

        <t-col :span="2" class="operation-container">
          <t-button theme="primary" type="submit" :style="{ marginLeft: 'var(--td-comp-margin-s)' }">
            {{ $t('components.commonTable.query') }}
          </t-button>
          <t-button type="reset" variant="base" theme="default"> {{ $t('components.commonTable.reset') }}</t-button>
        </t-col>
      </t-row>
    </t-form>

    <div class="table-container">
      <t-space align="center">
        <t-button @click="handleClickNew">{{ $t('pages.rss.option.create') }}</t-button>
        <t-button theme="danger" @click="handleClickTruncate">{{ $t('pages.rss.option.truncate') }}</t-button>
      </t-space>

      <t-table :data="data" :columns="COLUMNS" :hover="true" :pagination="pagination" :loading="dataLoading">
        <template #status="{ row }">
          <t-switch v-model="row.enable" size="large" />
        </template>
        <template #op="slotProps">
          <t-space>
            <t-link theme="primary" @click="handleClickDetail(slotProps)"> {{ $t('pages.rss.option.detail') }}</t-link>
            <t-link theme="danger" @click="handleClickDelete(slotProps)"> {{ $t('pages.rss.option.delete') }}</t-link>
          </t-space>
        </template>
      </t-table>
      <t-dialog
        v-model:visible="deleteConfirmVisible"
        :header="$t('pages.rss.hint.delete')"
        :on-cancel="resetIdx"
        @confirm="onConfirmDelete"
      />
    </div>

    <t-dialog
      v-model:visible="truncateConfirmVisible"
      :header="$t('pages.rss.hint.truncate')"
      @confirm="onConfirmTruncate"
    />
    <dialog-form v-model:visible="formDialogVisible" :data="formData" @success="fetchData" />
  </div>
</template>
<script setup lang="ts">
import { MessagePlugin, PrimaryTableCol } from 'tdesign-vue-next';
import { onMounted, ref } from 'vue';

import { IndexerInfo, SearchParam } from '@/api/model/rssModel';
import { deleteIndexer, getIndexerList, truncateIndexer } from '@/api/rss';
import { t } from '@/locales';

import DialogForm from './DialogForm.vue';

const RSS_CATEGORY_OPTIONS = [
  { value: 'rss', label: 'rss' },
  { value: 'torznab', label: 'torznab' },
];

const COLUMNS: PrimaryTableCol[] = [
  {
    title: t('pages.rss.field.name'),
    fixed: 'left',
    width: 160,
    ellipsis: true,
    align: 'left',
    colKey: 'name',
  },
  { title: t('pages.rss.field.category'), colKey: 'category', width: 110 },
  { title: t('pages.rss.field.status'), colKey: 'status', width: 110, align: 'center' },
  {
    title: t('pages.rss.field.url'),
    fixed: 'left',
    ellipsis: true,
    align: 'left',
    colKey: 'url',
  },
  {
    align: 'left',
    fixed: 'right',
    width: 120,
    colKey: 'op',
    title: t('pages.rss.field.operation'),
  },
];

const INITIAL_DATA: IndexerInfo = {
  id: undefined,
  name: '',
  category: '',
  url: '',
  enable: true,
};

const INITIAL_PARAM: SearchParam = {
  category: undefined,
  name: undefined,
};

const formDialogVisible = ref(false);
const formData = ref({ ...INITIAL_DATA });
const searchParam = ref<SearchParam>({ ...INITIAL_PARAM });

const pagination = ref({
  defaultPageSize: 10,
  total: 0,
  defaultCurrent: 1,
});
const deleteConfirmVisible = ref(false);
const truncateConfirmVisible = ref(false);

const data = ref([]);
const dataLoading = ref(false);
const deleteIdx = ref(-1);

const fetchData = async () => {
  dataLoading.value = true;
  try {
    const list = await getIndexerList(searchParam.value);
    data.value = list;
    pagination.value = {
      ...pagination.value,
      total: list.length,
    };
  } catch (message) {
    await MessagePlugin.error(message);
  } finally {
    dataLoading.value = false;
  }
};

const searchReset = async () => {
  searchParam.value = { ...INITIAL_PARAM };
  await fetchData();
};

const resetIdx = () => {
  deleteIdx.value = -1;
};

const onConfirmDelete = async () => {
  try {
    await deleteIndexer(deleteIdx.value);
    deleteConfirmVisible.value = false;
    await MessagePlugin.success(t('pages.rss.hint.delete_success'));
    await fetchData();
    resetIdx();
  } catch (message) {
    await MessagePlugin.error(message);
  }
};

const onConfirmTruncate = async () => {
  try {
    await truncateIndexer();
    truncateConfirmVisible.value = false;
    await MessagePlugin.success(t('pages.rss.hint.delete_success'));
    await fetchData();
  } catch (message) {
    await MessagePlugin.error(message);
  }
};

onMounted(() => {
  fetchData();
});

const handleClickNew = () => {
  formDialogVisible.value = true;
};

const handleClickTruncate = () => {
  truncateConfirmVisible.value = true;
};

const handleClickDelete = (slot: { row: { id: number } }) => {
  deleteIdx.value = slot.row.id;
  deleteConfirmVisible.value = true;
};

const handleClickDetail = (slot: { row: IndexerInfo }) => {
  formData.value = { ...slot.row };
  formDialogVisible.value = true;
};
</script>

<style lang="less" scoped>
.list-common-table {
  background-color: var(--td-bg-color-container);
  padding: var(--td-comp-paddingTB-xxl) var(--td-comp-paddingLR-xxl);
  border-radius: var(--td-radius-medium);

  .table-container {
    margin-top: var(--td-comp-margin-xxl);
  }
}

.form-item-content {
  width: 100%;
}

.operation-container {
  display: flex;
  justify-content: flex-end;
  align-items: center;

  .expand {
    .t-button__text {
      display: flex;
      align-items: center;
    }
  }
}
</style>
