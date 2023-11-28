<template>
  <t-form ref="form" class="item-container" :data="formData" :rules="FORM_RULES" label-width="0" @submit="onSubmit">
    <t-form-item name="account">
      <t-input v-model="formData.account" size="large" :placeholder="`${$t('pages.login.input.account')}`">
        <template #prefix-icon>
          <user-icon />
        </template>
      </t-input>
    </t-form-item>

    <t-form-item name="password">
      <t-input
        v-model="formData.password"
        size="large"
        :type="showPsw ? 'text' : 'password'"
        clearable
        :placeholder="`${$t('pages.login.input.password')}`"
      >
        <template #prefix-icon>
          <lock-on-icon />
        </template>
        <template #suffix-icon>
          <browse-icon v-if="showPsw" @click="showPsw = !showPsw" />
          <browse-off-icon v-else @click="showPsw = !showPsw" />
        </template>
      </t-input>
    </t-form-item>

    <t-form-item class="btn-container">
      <t-button block size="large" type="submit"> {{ $t('pages.login.signIn') }}</t-button>
    </t-form-item>
  </t-form>
</template>

<script setup lang="ts">
import { BrowseIcon, BrowseOffIcon, LockOnIcon, UserIcon } from 'tdesign-icons-vue-next';
import type { FormInstanceFunctions, FormRule, SubmitContext } from 'tdesign-vue-next';
import { MessagePlugin } from 'tdesign-vue-next';
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import { LoginParam } from '@/api/model/authModel';
import { t } from '@/locales';
import { useUserStore } from '@/store';

const userStore = useUserStore();

const INITIAL_DATA = {
  account: 'admin',
  password: 'admin123',
};

const FORM_RULES: Record<string, FormRule[]> = {
  account: [{ required: true, message: t('pages.login.required.account'), type: 'error' }],
  password: [{ required: true, message: t('pages.login.required.password'), type: 'error' }],
};

const form = ref<FormInstanceFunctions>();
const formData = ref<LoginParam>({ ...INITIAL_DATA });
const showPsw = ref(false);

const router = useRouter();
const route = useRoute();

const onSubmit = async (ctx: SubmitContext) => {
  if (ctx.validateResult === true) {
    try {
      await userStore.login(formData.value);
      await MessagePlugin.success(t('pages.login.result.success'));
      const redirect = route.query.redirect as string;
      const redirectUrl = redirect ? decodeURIComponent(redirect) : '/';
      await router.push(redirectUrl);
    } catch (code) {
      if (code === 421) {
        await MessagePlugin.error(t('pages.login.result.username_error'));
      } else if (code === 422) {
        await MessagePlugin.error(t('pages.login.result.password_error'));
      } else {
        await MessagePlugin.error(t('pages.login.result.other_error'));
      }
    }
  }
};
</script>

<style lang="less" scoped>
@import '../index.less';
</style>
