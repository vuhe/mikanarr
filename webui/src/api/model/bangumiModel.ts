export interface BangumiListResult {
  list: Array<BangumiInfo>;
}

export interface BangumiInfo {
  id: number;
  title: string;
  poster: string;
  year: number | undefined;
  season: number;
  offset: number;
  enable: boolean;
  exclude: Array<string>;
}
