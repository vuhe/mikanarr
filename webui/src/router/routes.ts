import { Film1Icon, RssIcon } from 'tdesign-icons-vue-next';
import { shallowRef } from 'vue';
import { RouteRecordRaw } from 'vue-router';

import Layout from '@/layouts/index.vue';

export const routerList: Array<RouteRecordRaw> = [
  {
    path: '/bangumi',
    component: Layout,
    redirect: '/bangumi/list',
    name: 'bangumi',
    meta: {
      title: {
        zh_CN: '番剧追踪',
        en_US: 'Bangumi Tracking',
      },
      icon: shallowRef(Film1Icon),
      orderNo: 0,
      single: true,
    },
    children: [
      {
        path: 'list',
        name: 'BangumiList',
        component: () => import('@/pages/bangumi/index.vue'),
        meta: {
          title: {
            zh_CN: '番剧列表',
            en_US: 'BangumiList',
          },
        },
      },
    ],
  },
  {
    path: '/rss',
    component: Layout,
    redirect: '/rss/list',
    name: 'rss',
    meta: {
      title: {
        zh_CN: '订阅管理',
        en_US: 'Subscription',
      },
      icon: shallowRef(RssIcon),
      orderNo: 1,
      single: true,
    },
    children: [
      {
        path: 'list',
        name: 'RssList',
        component: () => import('@/pages/rss/index.vue'),
        meta: {
          title: {
            zh_CN: 'RSS 列表',
            en_US: 'RssList',
          },
        },
      },
    ],
  },
  {
    path: '/result',
    name: 'result',
    component: Layout,
    redirect: '/result/success',
    meta: {
      title: {
        zh_CN: '结果页',
        en_US: 'Result',
      },
      icon: 'check-circle',
      orderNo: 100,
    },
    children: [
      {
        path: 'success',
        name: 'ResultSuccess',
        component: () => import('@/pages/result/success/index.vue'),
        meta: {
          title: {
            zh_CN: '成功页',
            en_US: 'Success',
          },
        },
      },
      {
        path: 'fail',
        name: 'ResultFail',
        component: () => import('@/pages/result/fail/index.vue'),
        meta: {
          title: {
            zh_CN: '失败页',
            en_US: 'Fail',
          },
        },
      },
      {
        path: 'network-error',
        name: 'ResultNetworkError',
        component: () => import('@/pages/result/network-error/index.vue'),
        meta: {
          title: {
            zh_CN: '网络异常',
            en_US: 'Network Error',
          },
        },
      },
      {
        path: '403',
        name: 'Result403',
        component: () => import('@/pages/result/403/index.vue'),
        meta: { title: { zh_CN: '无权限', en_US: 'Forbidden' } },
      },
      {
        path: '404',
        name: 'Result404',
        component: () => import('@/pages/result/404/index.vue'),
        meta: { title: { zh_CN: '访问页面不存在页', en_US: 'Not Found' } },
      },
      {
        path: '500',
        name: 'Result500',
        component: () => import('@/pages/result/500/index.vue'),
        meta: { title: { zh_CN: '服务器出错页', en_US: 'Server Error' } },
      },
      {
        path: 'browser-incompatible',
        name: 'ResultBrowserIncompatible',
        component: () => import('@/pages/result/browser-incompatible/index.vue'),
        meta: { title: { zh_CN: '浏览器不兼容页', en_US: 'BrowserIncompatible' } },
      },
      {
        path: 'maintenance',
        name: 'ResultMaintenance',
        component: () => import('@/pages/result/maintenance/index.vue'),
        meta: { title: { zh_CN: '系统维护页', en_US: 'Maintenance' } },
      },
    ],
  },
  {
    path: '/setting',
    name: 'setting',
    component: Layout,
    redirect: '/setting/index',
    meta: {
      title: {
        zh_CN: '系统设置',
        en_US: 'Setting',
      },
      icon: 'setting-1',
      orderNo: 50,
      single: true,
    },
    children: [
      {
        path: 'index',
        name: 'SettingIndex',
        component: () => import('@/pages/setting/index.vue'),
        meta: { title: { zh_CN: '系统设置', en_US: 'Setting' } },
      },
    ],
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/pages/login/index.vue'),
  },
  {
    path: '/',
    redirect: '/bangumi/list',
  },
  {
    path: '/:catchAll(.*)',
    redirect: '/result/404',
  },
];
