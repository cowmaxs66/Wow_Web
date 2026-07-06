# dm-bridge 说明

## 职责
大漠插件桥接层，后续使用 Delphi DLL 通过 C ABI 暴露稳定函数给 Rust 调用。

## 当前状态
P2-S02 进行中。当前先完成 C ABI 契约设计，不创建占位 DLL 代码，避免未验证的 COM 调用模型污染主干。

## P2 前必须明确
- 字符串编码。
- 内存分配与释放归属。
- COM 初始化和释放规则。
- STA 单线程调用队列。
- 错误码与错误消息结构。

## 计划目录排版
后续 Delphi 工程必须按职责拆分，禁止把所有导出函数写在一个文件中。

| 文件 | 职责 |
|------|------|
| `DmBridge.dpr` | DLL 工程入口和 exports |
| `DmBridge.Types.pas` | C ABI record、常量、状态码 |
| `DmBridge.Strings.pas` | UTF-16 输入输出和 buffer 写入 |
| `DmBridge.Errors.pas` | Bridge 错误码和错误消息 |
| `DmBridge.Worker.pas` | STA Worker 线程和同步调用队列 |
| `DmBridge.Dmsoft.pas` | 大漠 COM 对象封装 |
| `DmBridge.Api.*.pas` | 分领域实现基本、窗口、后台、图色、键鼠、算法接口 |

## 契约文档
详见：`技术设计/DmBridge_C_ABI契约设计.md`
