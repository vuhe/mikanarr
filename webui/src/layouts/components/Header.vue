<template>
  <div :class="layoutCls">
    <t-head-menu :class="menuCls" :theme="menuTheme" expand-type="popup" :value="active">
      <template #logo>
        <div class="header-operate-left">
          <t-button theme="default" shape="square" variant="text" @click="changeCollapsed">
            <view-list-icon class="collapsed-icon" />
          </t-button>
        </div>
      </template>
      <template #operations>
        <div class="operations-container">
          <t-tooltip placement="bottom" :content="$t('layout.header.code')">
            <t-button theme="default" shape="square" variant="text" @click="navToGitHub">
              <logo-github-icon />
            </t-button>
          </t-tooltip>
          <t-tooltip placement="bottom" :content="$t('layout.header.help')">
            <t-button theme="default" shape="square" variant="text" @click="navToHelper">
              <help-circle-icon />
            </t-button>
          </t-tooltip>
          <t-dropdown trigger="click">
            <t-button theme="default" shape="square" variant="text">
              <mode-light-icon v-if="menuTheme === 'light'" />
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
          <t-dropdown :min-column-width="120" trigger="click">
            <template #dropdown>
              <t-dropdown-menu>
                <t-dropdown-item class="operations-dropdown-container-item" @click="handleLogout">
                  <poweroff-icon />
                  {{ $t('layout.header.signOut') }}
                </t-dropdown-item>
              </t-dropdown-menu>
            </template>
            <t-button class="header-user-btn" theme="default" variant="text">
              <template #icon>
                <user-circle-icon class="header-user-avatar" />
              </template>
              <div class="header-user-account">{{ user.userInfo.name }}</div>
              <template #suffix>
                <chevron-down-icon />
              </template>
            </t-button>
          </t-dropdown>
        </div>
      </template>
    </t-head-menu>
  </div>
</template>

<script setup lang="ts">
import {
  ChevronDownIcon,
  HelpCircleIcon,
  LogoGithubIcon,
  ModeDarkIcon,
  ModeLightIcon,
  PoweroffIcon,
  TranslateIcon,
  UserCircleIcon,
  ViewListIcon,
} from 'tdesign-icons-vue-next';
import { computed } from 'vue';
import { useRouter } from 'vue-router';

import { prefix } from '@/config/global';
import { langList } from '@/locales';
import { useLocale } from '@/locales/useLocale';
import { getActive } from '@/router';
import { useSettingStore, useUserStore } from '@/store';
import { ModeType } from '@/types/interface';

const props = defineProps({
  theme: {
    type: String,
    default: 'light',
  },
  maxLevel: {
    type: Number,
    default: 3,
  },
});

const router = useRouter();
const settingStore = useSettingStore();
const user = useUserStore();

const active = computed(() => getActive());

const layoutCls = computed(() => [`${prefix}-header-layout`]);

const menuCls = computed(() => {
  return [
    {
      [`${prefix}-header-menu-fixed`]: true,
      [`${prefix}-header-menu-fixed-side`]: true,
      [`${prefix}-header-menu-fixed-side-compact`]: settingStore.isSidebarCompact,
    },
  ];
});
const menuTheme = computed(() => props.theme as 'light' | 'dark');

// 切换语言
const { changeLocale } = useLocale();
const changeLang = ({ value: lang }: { value: string }) => {
  changeLocale(lang);
};

// 切换主题
const changeTheme = async ({ value: theme }: { value: ModeType }) => {
  await settingStore.changeMode(theme);
};

const changeCollapsed = () => {
  settingStore.updateConfig({
    isSidebarCompact: !settingStore.isSidebarCompact,
  });
};

const handleLogout = () => {
  router.push({
    path: '/login',
    query: { redirect: encodeURIComponent(router.currentRoute.value.fullPath) },
  });
};

const navToGitHub = () => {
  window.open('https://github.com/vuhe/mikanarr');
};

const navToHelper = () => {
  window.open('https://github.com/vuhe/mikanarr/wiki');
};
</script>
<style lang="less" scoped>
.@{starter-prefix}-header {
  &-menu-fixed {
    position: fixed;
    top: 0;
    z-index: 1001;

    :deep(.t-head-menu__inner) {
      padding-right: var(--td-comp-margin-xl);
    }

    &-side {
      left: 232px;
      right: 0;
      z-index: 10;
      width: auto;
      transition: all 0.3s;

      &-compact {
        left: 64px;
      }
    }
  }

  &-logo-container {
    cursor: pointer;
    display: inline-flex;
  }
}

.header-menu {
  flex: 1 1 1;
  display: inline-flex;

  :deep(.t-menu__item) {
    min-width: unset;
  }
}

.operations-container {
  display: flex;
  align-items: center;

  .t-popup__reference {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .t-button {
    margin-left: var(--td-comp-margin-l);
  }
}

.header-operate-left {
  display: flex;
  align-items: normal;
  line-height: 0;
  padding-left: var(--td-comp-margin-xl);
}

.header-logo-container {
  width: 184px;
  height: 26px;
  display: flex;
  margin-left: 24px;
  color: var(--td-text-color-primary);

  .t-logo {
    width: 100%;
    height: 100%;

    &:hover {
      cursor: pointer;
    }
  }

  &:hover {
    cursor: pointer;
  }
}

.header-user-account {
  display: inline-flex;
  align-items: center;
  color: var(--td-text-color-primary);
}

:deep(.t-head-menu__inner) {
  border-bottom: 1px solid var(--td-component-stroke);
}

.t-menu--light {
  .header-user-account {
    color: var(--td-text-color-primary);
  }
}

.t-menu--dark {
  .t-head-menu__inner {
    border-bottom: 1px solid var(--td-gray-color-10);
  }

  .header-user-account {
    color: rgb(255 255 255 / 55%);
  }
}

.operations-dropdown-container-item {
  width: 100%;
  display: flex;
  align-items: center;

  :deep(.t-dropdown__item-text) {
    display: flex;
    align-items: center;
  }

  .t-icon {
    font-size: var(--td-comp-size-xxxs);
    margin-right: var(--td-comp-margin-s);
  }

  :deep(.t-dropdown__item) {
    width: 100%;
    margin-bottom: 0;
  }

  &:last-child {
    :deep(.t-dropdown__item) {
      margin-bottom: 8px;
    }
  }
}
</style>

<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -->
<style lang="less">
.operations-dropdown-container-item {
  .t-dropdown__item-text {
    display: flex;
    align-items: center;
  }
}
</style>
