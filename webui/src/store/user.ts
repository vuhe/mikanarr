import { defineStore } from 'pinia';

import { login as loginApi, username as usernameApi } from '@/api/auth';
import { LoginParam } from '@/api/model/authModel';
import type { UserInfo } from '@/types/interface';

const InitUserInfo: UserInfo = {
  name: '', // 用户名，用于展示在页面右上角头像处
  roles: ['all'], // 前端权限模型使用，未启用
};

export const useUserStore = defineStore('user', {
  state: () => ({
    token: '',
    version: '',
    userInfo: { ...InitUserInfo },
  }),
  actions: {
    async login(userInfo: LoginParam) {
      const { code, data } = await loginApi(userInfo);
      if (code === 200) {
        this.token = data;
      } else {
        throw code;
      }
    },
    async getUserInfo() {
      const { username, version } = await usernameApi();
      this.userInfo.name = username;
      this.version = version;
    },
    async logout() {
      this.token = '';
      this.userInfo = { ...InitUserInfo };
    },
  },
  persist: {
    key: 'user',
    paths: ['token'],
  },
});
