<template>
  <t-form ref="form" :data="formData" :rules="rules" @submit="onSubmit">
    <div class="setting-operation">
      <t-space>
        <t-button theme="warning" @click="handleModifyPassword"
          >{{ $t('pages.setting.option.modify_password') }}
        </t-button>
        <t-button theme="primary" type="submit">{{ $t('pages.setting.option.save') }}</t-button>
      </t-space>
    </div>

    <t-row :gutter="[24, 24]">
      <t-col :flex="1">
        <t-card :title="$t('pages.setting.title.bangumi')">
          <t-form-item
            :help="$t('pages.setting.help.bangumi_default_status')"
            :label="$t('pages.setting.form.bangumi_default_status')"
            name="bangumi_default_status"
          >
            <t-radio-group v-model="formData.bangumi_default_status">
              <t-radio :value="true">{{ $t('pages.setting.status.enable') }}</t-radio>
              <t-radio :value="false">{{ $t('pages.setting.status.disable') }}</t-radio>
            </t-radio-group>
          </t-form-item>
        </t-card>
      </t-col>
      <t-col :flex="1">
        <t-card :title="$t('pages.setting.title.auth')">
          <t-form-item :label="$t('pages.setting.form.username')" name="username">
            <t-input v-model="formData.username" />
          </t-form-item>
          <t-form-item
            :help="$t('pages.setting.help.auth_intranet')"
            :label="$t('pages.setting.form.auth_intranet')"
            name="auth_intranet"
          >
            <t-radio-group v-model="formData.auth_intranet">
              <t-radio :value="true">{{ $t('pages.setting.status.enable') }}</t-radio>
              <t-radio :value="false">{{ $t('pages.setting.status.disable') }}</t-radio>
            </t-radio-group>
          </t-form-item>
        </t-card>
      </t-col>
    </t-row>

    <dialog-form v-model:visible="formDialogVisible" />
  </t-form>
</template>
<script setup lang="ts">
import { FormRules, MessagePlugin, SubmitContext } from 'tdesign-vue-next';
import { onMounted, ref } from 'vue';

import { Settings } from '@/api/model/settingModel';
import { getSettingInfo, modifySetting } from '@/api/setting';
import { t } from '@/locales';

import DialogForm from './DialogForm.vue';

const INITIAL_DATA: Settings = {
  bangumi_default_status: false,
  username: 'admin',
  password: undefined,
  auth_intranet: true,
};

const formDialogVisible = ref(false);
const formData = ref<Settings>({ ...INITIAL_DATA });

const onSubmit = async ({ firstError }: SubmitContext<FormData>) => {
  if (firstError) {
    await MessagePlugin.warning(firstError);
    return;
  }

  try {
    await modifySetting(formData.value);
    formData.value = await getSettingInfo();
    await MessagePlugin.success(t('pages.setting.hint.save_success'));
  } catch (message) {
    await MessagePlugin.error(message);
  }
};

const handleModifyPassword = () => {
  formDialogVisible.value = true;
};

onMounted(async () => {
  try {
    formData.value = await getSettingInfo();
  } catch (message) {
    await MessagePlugin.error(message);
  }
});

const rules: FormRules<FormData> = {};
</script>
<style lang="less" scoped>
.setting {
  height: 100%;

  &-operation {
    display: flex;
    justify-content: space-between;
    margin-bottom: var(--td-comp-margin-xxl);
  }
}
</style>
