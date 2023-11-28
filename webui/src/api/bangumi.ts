import type { BangumiListResult } from '@/api/model/bangumiModel';
import { request } from '@/utils/request';

const Api = {
  BangumiList: '/bangumi/list',
};

export function getBangumiList(_keyword: string) {
  // todo: keyword 不为空时进行字符串查询
  return request.get<BangumiListResult>({
    url: Api.BangumiList,
  });
}
