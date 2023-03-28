![demo2](demo1.png)

# Rust 企业级一站式后台解决方案

- 坚如磐石（Rust 语言），高性能，无 GC 无内存泄漏，无协程竞争
- DDD 领域驱动,Mysql,Redis，通用中间件和数据库，通用企业级框架选型
- [rbatis-orm](https://github.com/rbatis/rbatis) 和 Mybatis-Plus 一样的好用，简洁，易扩展的 ORM 框架
- [fast_log](https://github.com/rbatis/fast_log) 超快速异步日志框架，支持 zip 压缩，切割
- [actix-web](https://actix.rs/) 常年屠榜 web 框架压测网站的框架
- 前后端分离,基于 [Vue-JS](https://cn.vuejs.org/) +[Vue-AntDesign](https://www.antdv.com/docs/vue/introduce-cn/) + [Vue-AntDesign-Pro](https://pro.antdv.com/)
- RBAC 权限控制,自带 JwtToken 鉴权登陆，图形验证码登陆，二维码扫码登陆,基础权限管理

# 进度、功能模块（包含(包含 web 前端和 rust 后端)）

| 功能(包含 web 前端和 rust 后端)                          | 完成(√)、进行中(x) |
| -------------------------------------------------------- | ------------------ |
| 动态菜单(菜单路由表权限动态生成)                         | √                  |
| JWT 拦截器校验                                           | √                  |
| JWT 账号密码登陆                                         | √                  |
| 设置/权限管理（父子级，分菜单权限+按钮权限，缓存 redis） | √                  |
| 设置/角色管理（父子级,分层级权限树，缓存 redis）         | √                  |
| 设置/后台账号管理（分层角色树）                          | √                  |
| 设置/键值对常量管理                                      | √                  |
| JWT 图形验证码+密码登陆                                  | x                  |
| JWT 短信登陆（基于 redis 短信消息）                      | x                  |

# 此项目存在的意义

- 高性能，快如 C++, 超低内存占用，支持廉价服务器
- 稳定，部署无忧，无内存泄漏，无闪退
- 开箱即用

# （rust 服务器端安装）快速安装教程

- 1.abs_admin 启动指令 `cargo run`或者`cargo build`编译可执行文件执行
- 2.(可选)docker 命令快速启动 redis `docker run -it -d --name redis -p 6379:6379 redis`
- 3.(可选，默认数据库 Sqlite,mysql 需要 Cargo.toml 添加 rbdc_mysql 依赖，并修改 application.yml 中的 database_url)docker 命令快速启动 mysql(用户名 root 密码 123456)。生产 docker 可以建议部署 http 服务，原则上生产环境不建议用 docker 部署数据库 `docker run -d -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 --name mysql -e TZ=Asia/Shanghai mysql:5.7`
- 4.（rust 服务器端安装）使用 Clion Database 窗口 或者 MysqlWorkBench 或 Navicat 等工具 导入 database.sql 脚本到 数据库
~~~~

# module(模块)

- JWT token Auth(基于 JWT token 的权限鉴权)
- Role,User,Reource（角色，用户，权限）
