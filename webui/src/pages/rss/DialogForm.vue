<template>
  <t-dialog v-model:visible="formVisible" :header="title" :width="680" :footer="false">
    <template #body>
      <t-form ref="form" :data="formData" :rules="rules" :label-width="100" @submit="onSubmit">
        <t-form-item :label="$t('pages.rss.field.name')" name="name">
          <t-input v-model="formData.name" :style="{ width: '480px' }" />
        </t-form-item>
        <t-form-item :label="$t('pages.rss.field.category')" name="category">
          <t-select v-model="formData.category" clearable :style="{ width: '480px' }">
            <t-option v-for="(item, index) in SELECT_OPTIONS" :key="index" :value="item.value" :label="item.label">
              {{ item.label }}
            </t-option>
          </t-select>
        </t-form-item>
        <t-form-item :label="$t('pages.rss.field.url')" name="url">
          <t-input v-model="formData.url" :style="{ width: '480px' }" />
        </t-form-item>
        <t-form-item :label="$t('pages.rss.field.status')" name="enable">
          <t-radio-group v-model="formData.enable" :default-value="true">
            <t-radio :value="true">{{ $t('pages.rss.status.enable') }}</t-radio>
            <t-radio :value="false">{{ $t('pages.rss.status.disable') }}</t-radio>
          </t-radio-group>
        </t-form-item>
        <t-form-item style="float: right">
          <t-button variant="outline" @click="onClickCloseBtn">{{ $t('pages.rss.option.cancel') }}</t-button>
          <t-button theme="primary" type="submit">{{ $t('pages.rss.option.submit') }}</t-button>
        </t-form-item>
      </t-form>
    </template>
  </t-dialog>
</template>

<script setup lang="ts">
import { FormRules, MessagePlugin, SubmitContext } from 'tdesign-vue-next';
import type { PropType } from 'vue';
import { computed, ref, watch } from 'vue';

import { IndexerInfo } from '@/api/model/rssModel';
import { addIndexer, modifyIndexer } from '@/api/rss';
import { t } from '@/locales';

const INITIAL_DATA: IndexerInfo = {
  id: undefined,
  name: '',
  category: '',
  url: '',
  enable: true,
};

const SELECT_OPTIONS = [
  { label: 'rss', value: 'rss' },
  { label: 'torznab', value: 'torznab' },
];

const emit = defineEmits(['update:visible', 'success']);
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  data: Object as PropType<IndexerInfo>,
});
const form = ref(null);
const formVisible = ref(false);
const formData = ref<IndexerInfo>({ ...INITIAL_DATA });

const title = computed(() => {
  if (formData.value.id) {
    return t('pages.rss.hint.modify');
  }
  return t('pages.rss.hint.create');
});

const onSubmit = async ({ firstError }: SubmitContext<IndexerInfo>) => {
  if (firstError) {
    await MessagePlugin.warning(firstError);
    return;
  }
  try {
    if (formData.value.id) {
      await modifyIndexer(formData.value);
    } else {
      await addIndexer(formData.value);
    }
    emit('success');
    formVisible.value = false;
    form.value.reset();
  } catch (message) {
    await MessagePlugin.error(message);
  }
};

const onClickCloseBtn = () => {
  formVisible.value = false;
  form.value.reset();
};

watch(
  () => formVisible.value,
  (val) => {
    emit('update:visible', val);
  },
);

watch(
  () => props.visible,
  (val) => {
    formVisible.value = val;
  },
);

watch(
  () => props.data,
  (val) => {
    formData.value = val;
  },
);

const rules: FormRules<IndexerInfo> = {
  name: [{ required: true, message: t('pages.rss.placeholder.name'), type: 'error' }],
  category: [{ required: true, message: t('pages.rss.placeholder.category'), type: 'error' }],
  url: [{ required: true, message: t('pages.rss.placeholder.url'), type: 'error' }],
  enable: [{ required: true, message: t('pages.rss.placeholder.enable'), type: 'error' }],
};
</script>
