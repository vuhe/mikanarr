import { defineStore } from 'pinia';

import { allRoutes } from '@/router';
import { store } from '@/store';

export const usePermissionStore = defineStore('permission', {
  state: () => ({
    whiteListRouters: ['/login'],
    routers: [...allRoutes],
  }),
});

export function getPermissionStore() {
  return usePermissionStore(store);
}
