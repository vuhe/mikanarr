import { Settings } from '@/api/model/settingModel';
import { request } from '@/utils/request';

const Api = {
  SettingInfo: '/setting/info',
  ModifySetting: '/setting/modify',
};

export function getSettingInfo() {
  return request.get<Settings>({
    url: Api.SettingInfo,
  });
}

export function modifySetting(indexer: Settings) {
  return request.put<void>({
    url: Api.ModifySetting,
    data: indexer,
  });
}
