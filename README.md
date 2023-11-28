<h1 style="text-align:center">mikanarr</h1>

## 项目说明

> 项目施工中……
> 
> 项目仅供学习 rust 使用，请勿在任何社交平台发布传播本项目相关内容

本项目可以解析 RSS 订阅，并按规则进行下载和重命名，重命名后的文件路径可以直接被 PVR 软件识别。

本项目是作为 `rust` + `vue` 技术的学习项目，如果您查阅、下载了本项目源代码或二进制程序，即代表你接受了以下条款：

- 本项目和项目成果仅供技术、学术交流和 `rust` 性能测试使用
- 使用者必须自行确保获取影片的途径在使用者当地是合法的
- 程序获取的元数据和封面图片等数据的版权，归版权持有人持有
- 本项目贡献者编写该项目旨在学习 `rust` 或 `vue` ，提高编程水平
- 本项目不提供任何影片下载的方法及途径
- 请勿提供运行时和运行后获取的数据提供给可能有非法目的的第三方，例如用于非法交易、侵犯未成年人的权利等
- 用户仅能在自己的私人计算机或者测试环境中使用该工具，禁止将获取到的数据用于商业目的或其他目的，如销售、传播等
- 用户在使用本项目和项目成果前，请用户了解并遵守当地法律法规，如果本项目及项目成果使用过程中存在违反当地法律法规的行为，请勿使用该项目及项目成果
- 法律后果及使用后果由使用者承担
- 使用本项目需要遵循 [GPL-3.0](https://github.com/vuhe/mikanarr/blob/main/LICENSE) 许可

若您不同意上述条款任意一条，请勿使用本项目和项目成果

## 本地运行

> 请确保本地运行环境中可以使用 cargo 和 npm 命令

1. 设置启动环境变量 `MK_TMDB_API` 用于访问 tmdb api
2. 检查确保 `7810` 端口未被占用
3. 执行 `cargo run --package mikanarr --bin mikanarr` 启动后端 
4. 检查确保 `3002` 端口未被占用，并执行 `cd webui` 切换至 `webui` 目录
5. 执行 `npm install` 安装前端依赖
6. 执行 `npm run dev` 启动前端

本地运行所产生的数据默认存放于同目录下 `.data` 文件夹，可以通过 `MK_APP_DATA` 环境变量进行调整

## 打包运行

> 请确保本地运行环境中可以运行 docker 并已开启 Buildkit

将您的 tmdb api 替换下面命令中的 `{tmdb_api}`，

运行下方命令执行 docker 构建

```shell
DOCKER_BUILDKIT=1 \
docker build \
--build-arg MK_TMDB_API={tmdb_api} \
-t mikanarr:latest .
```

## 参与贡献

您可以随意提交 issues 和 PR，由于本项目仅作为 rust 学习使用，因此请遵守以下限制：

- 使用相关 issues 会被直接关闭，项目不提供使用说明
- 出于学习考虑，提交 PR 需要说明理由（简单注明修复或提供什么功能即可）
