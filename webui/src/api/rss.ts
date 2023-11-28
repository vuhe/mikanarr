import { IndexerInfo, SearchParam } from '@/api/model/rssModel';
import { request } from '@/utils/request';

const Api = {
  IndexerList: '/indexer/list',
  AddIndexer: '/indexer/add',
  ModifyIndexer: '/indexer/modify',
  DeleteIndexer: '/indexer/delete',
  TruncateIndexer: '/indexer/truncate',
};

export function getIndexerList(param: SearchParam) {
  return request.get<Array<IndexerInfo>>({
    url: Api.IndexerList,
    params: param,
  });
}

export function addIndexer(indexer: IndexerInfo) {
  return request.post<void>({
    url: Api.AddIndexer,
    data: indexer,
  });
}

export function modifyIndexer(indexer: IndexerInfo) {
  return request.put<void>({
    url: Api.ModifyIndexer,
    data: indexer,
  });
}

export function deleteIndexer(id: number) {
  return request.delete<void>({
    url: Api.DeleteIndexer,
    data: { id },
  });
}

export function truncateIndexer() {
  return request.delete<void>({
    url: Api.TruncateIndexer,
  });
}
