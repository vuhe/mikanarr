<template>
  <header class="login-header">
    <logo-full-icon class="logo" />
    <div class="operations-container">
      <t-button theme="default" shape="square" variant="text" @click="navToGitHub">
        <logo-github-icon class="icon" />
      </t-button>
      <t-button theme="default" shape="square" variant="text" @click="navToHelper">
        <help-circle-icon class="icon" />
      </t-button>
      <t-dropdown trigger="click">
        <t-button theme="default" shape="square" variant="text">
          <mode-light-icon v-if="displayMode === 'light'" />
          <mode-dark-icon v-else />
        </t-button>
        <t-dropdown-menu>
          <t-dropdown-item value="light" @click="changeTheme"
            >{{ $t('layout.setting.theme.options.light') }}
          </t-dropdown-item>
          <t-dropdown-item value="dark" @click="changeTheme"
            >{{ $t('layout.setting.theme.options.dark') }}
          </t-dropdown-item>
          <t-dropdown-item value="auto" @click="changeTheme"
            >{{ $t('layout.setting.theme.options.auto') }}
          </t-dropdown-item>
        </t-dropdown-menu>
      </t-dropdown>
      <t-dropdown trigger="click">
        <t-button theme="default" shape="square" variant="text">
          <translate-icon />
        </t-button>
        <t-dropdown-menu>
          <t-dropdown-item v-for="(lang, index) in langList" :key="index" :value="lang.value" @click="changeLang"
            >{{ lang.content }}
          </t-dropdown-item>
        </t-dropdown-menu>
      </t-dropdown>
    </div>
  </header>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { HelpCircleIcon, LogoGithubIcon, ModeDarkIcon, ModeLightIcon, TranslateIcon } from 'tdesign-icons-vue-next';

import LogoFullIcon from '@/assets/assets-logo-full.svg?component';
import { langList } from '@/locales';
import { useLocale } from '@/locales/useLocale';
import { useSettingStore } from '@/store';
import { ModeType } from '@/types/interface';

const settingStore = useSettingStore();
const { displayMode } = storeToRefs(settingStore);

// 切换语言
const { changeLocale } = useLocale();
const changeLang = ({ value: lang }: { value: string }) => {
  changeLocale(lang);
};

// 切换主题
const changeTheme = async ({ value: theme }: { value: ModeType }) => {
  await settingStore.changeMode(theme);
};

const navToGitHub = () => {
  window.open('https://github.com/vuhe/mikanarr');
};

const navToHelper = () => {
  window.open('https://github.com/vuhe/mikanarr/wiki');
};
</script>

<style lang="less" scoped>
.login-header {
  padding: 0 var(--td-comp-paddingLR-xl);
  display: flex;
  justify-content: space-between;
  align-items: center;
  backdrop-filter: blur(10px);
  color: var(--td-text-color-primary);
  height: var(--td-comp-size-xxxl);

  .logo {
    width: 178px;
    height: var(--td-comp-size-s);
  }

  .operations-container {
    display: flex;
    align-items: center;

    .t-button {
      margin-left: var(--td-comp-margin-l);
    }
  }
}
</style>
