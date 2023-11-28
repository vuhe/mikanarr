<template>
  <t-dialog v-model:visible="formVisible" :header="$t('pages.setting.title.password')" :width="680" :footer="false">
    <template #body>
      <t-form ref="form" :data="formData" :rules="rules" :label-width="150" @submit="onSubmit">
        <t-form-item :label="$t('pages.setting.form.password')" name="password">
          <t-input v-model="formData.password" type="password"></t-input>
        </t-form-item>
        <t-form-item :label="$t('pages.setting.form.r_password')" name="rePassword">
          <t-input v-model="formData.rePassword" type="password"></t-input>
        </t-form-item>
        <t-form-item style="float: right">
          <t-button variant="outline" @click="onClickCloseBtn">{{ $t('pages.setting.option.cancel') }}</t-button>
          <t-button theme="primary" type="submit">{{ $t('pages.setting.option.submit') }}</t-button>
        </t-form-item>
      </t-form>
    </template>
  </t-dialog>
</template>

<script setup lang="ts">
import { FormRules, MessagePlugin, SubmitContext } from 'tdesign-vue-next';
import { ref, watch } from 'vue';

import { IndexerInfo } from '@/api/model/rssModel';
import { Settings } from '@/api/model/settingModel';
import { modifySetting } from '@/api/setting';
import { t } from '@/locales';

interface PasswordSettings extends Settings {
  rePassword: string | undefined;
}

const INITIAL_DATA: PasswordSettings = {
  bangumi_default_status: undefined,
  username: undefined,
  password: undefined,
  rePassword: undefined,
  auth_intranet: undefined,
};

const emit = defineEmits(['update:visible']);
const props = defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
});
const form = ref(null);
const formVisible = ref(false);
const formData = ref<PasswordSettings>({ ...INITIAL_DATA });

const onSubmit = async ({ firstError }: SubmitContext<IndexerInfo>) => {
  if (firstError) {
    await MessagePlugin.warning(firstError);
    return;
  }
  try {
    await modifySetting(formData.value);
    formVisible.value = false;
    await MessagePlugin.success(t('pages.setting.hint.save_success'));
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

const rePassword = (val: any) => {
  return formData.value.password === val;
};

const rules: FormRules<PasswordSettings> = {
  password: [{ required: true, message: t('pages.setting.placeholder.password'), type: 'error' }],
  rePassword: [
    { required: true, message: t('pages.setting.placeholder.password'), type: 'error' },
    { validator: rePassword, message: t('pages.setting.placeholder.re_password') },
  ],
};
</script>
