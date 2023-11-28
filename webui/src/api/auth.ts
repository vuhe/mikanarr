import type { LoginParam, UserInfo } from '@/api/model/authModel';
import { Result } from '@/types/axios';
import { request } from '@/utils/request';

const Api = {
  Login: '/login',
  Username: '/username',
};

export function login(param: LoginParam) {
  return request.post<Result<string>>(
    {
      url: Api.Login,
      data: param,
    },
    {
      isTransformResponse: false,
    },
  );
}

export function username() {
  return request.get<UserInfo>({
    url: Api.Username,
  });
}
