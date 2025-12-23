# 一键发布工具

## todo

- [ ] 图文发布
- [ ] 指纹浏览器
- [ ] 本地数据库储存

## issue

- tauri打包问题

```javascript
// 打包时会报错，提示缺少依赖
// 解决方法：在tauri.conf.json中添加依赖
 "build": {
    "beforeDevCommand": "pnpm dev:tauri",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "pnpm build:tauri",
    "frontendDist": "../dist"
  },
```

不支持pnpm build && pnpm md:build这种写法，在运行pnpm tauri build时之后运行&之前的判断

- 打包时安装缓慢的问题
  打包时会下载github上的包，下载缓慢，可采用手动下载的方式
  [https://blog.csdn.net/weixin_43108331/article/details/146558922]
  [https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-bundler/src/bundle/windows/nsis/mod.rs]
