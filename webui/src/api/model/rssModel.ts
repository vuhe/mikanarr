export interface SearchParam {
  name: string | undefined;
  category: string | undefined;
}

export interface IndexerInfo {
  id: number | undefined;
  name: string;
  category: string;
  url: string;
  enable: boolean;
}
