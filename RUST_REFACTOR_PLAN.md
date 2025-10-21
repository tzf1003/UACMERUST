# UACME Rust 重构工作计划

创建时间：2025-10-21
评估结果：
- **理解深度**：高 - 需要深入理解整个项目的架构、功能模块和业务逻辑
- **变更范围**：系统 - 完整的语言迁移，涉及所有模块
- **风险等级**：高 - 语言转换可能导致功能遗漏或行为差异

**响应模式**：协作规划模式

---

## 项目概述

### 原项目结构分析
UACME 是一个 Windows UAC 绕过工具集，包含以下模块：

1. **Akagi** - 主可执行文件（x64/x86-32）
   - 入口点：`main.c`
   - 核心功能：方法调度、上下文管理、payload 加载
   - 包含 60+ 种 UAC 绕过方法

2. **Fubuki** - 通用 payload DLL（x64/x86-32）
   - 入口点：`dllmain.c`
   - 功能：执行提权后的 payload、UI 访问加载器

3. **Akatsuki** - WOW64 logger payload（x64）
   - 专用于 WOW64 日志方法
   - 提权到 NT AUTHORITY\SYSTEM

4. **Kamikaze** - MMC snap-in 数据单元

5. **Naka** - 压缩工具（x64/x86-32）
   - 用于压缩其他 payload/data 单元

6. **Yuubari** - UAC 信息转储工具（x64）
   - 转储 UAC 相关信息

7. **Shared** - 共享代码库
   - 字符串操作、PE 加载器、命令行解析、Windows 内部结构

### 核心技术特性
- 直接使用 Windows NT API
- 自定义 PE 加载器
- 方法调度表机制
- 共享参数块（进程间通信）
- Payload 压缩/解压
- 反模拟器检测
- COM 接口操作
- 文件系统操作（IFileOperation）
- 注册表操作
- 进程伪装

---

## 执行计划

### 阶段 1：项目基础设施搭建（预计 30 分钟）
- [x] 创建 Rust workspace 结构
- [ ] 配置 Cargo.toml（workspace 和各子项目）
- [ ] 设置构建配置（x64/x86 支持）
- [ ] 添加必要的依赖项

### 阶段 2：共享库重构（预计 2 小时）
- [ ] 字符串操作函数（`_strlen`, `_strcpy`, `_strcat` 等）
- [ ] Windows NT API 绑定（使用 `windows-rs` 或 `ntapi`）
- [ ] PE 加载器（`ldr.c` -> Rust）
- [ ] 命令行解析（`cmdline.c` -> Rust）
- [ ] 工具函数（`util.c` -> Rust）
- [ ] 反模拟器检测（`windefend.c` -> Rust）

### 阶段 3：核心模块重构（预计 3 小时）
- [ ] Akagi 主程序
  - [ ] 上下文管理（`sup.c`）
  - [ ] 方法调度表（`methods.c`）
  - [ ] Payload 加载机制
  - [ ] 共享参数块
  - [ ] 控制台输出
  
### 阶段 4：UAC 绕过方法迁移（预计 5 小时）
- [ ] 方法分类和优先级排序
- [ ] 高优先级方法（仍然有效的方法）
  - [ ] Method 22: SXS Consent
  - [ ] Method 23: DISM
  - [ ] Method 30: WOW64 Logger
  - [ ] Method 32: UI Access
  - [ ] Method 33: MS Settings
  - [ ] Method 34: Disk Silent Cleanup
  - [ ] Method 36: Junction
  - [ ] Method 37: SXS Dccw
  - [ ] Method 38: Hakril
  - [ ] Method 39: Cor Profiler
  - [ ] Method 41: CMLuaUtil
  - [ ] Method 43: Dccw COM
  - [ ] 其他活跃方法...
- [ ] 中优先级方法（部分系统有效）
- [ ] 低优先级方法（已修复但保留用于研究）

### 阶段 5：Payload 模块重构（预计 2 小时）
- [ ] Fubuki DLL
  - [ ] DLL 入口点
  - [ ] Payload 执行逻辑
  - [ ] UI 访问加载器
  - [ ] PCA 加载器
- [ ] Akatsuki DLL
  - [ ] WOW64 logger 特定逻辑
  - [ ] System 权限提升

### 阶段 6：辅助工具重构（预计 1.5 小时）
- [ ] Naka 压缩工具
- [ ] Yuubari UAC 信息转储工具

### 阶段 7：测试与验证（预计 2 小时）
- [ ] 单元测试
- [ ] 集成测试
- [ ] 在不同 Windows 版本上测试
- [ ] 验证关键方法的功能

### 阶段 8：文档与优化（预计 1 小时）
- [ ] 更新 README.md
- [ ] 添加 Rust 特定的构建说明
- [ ] 代码注释和文档
- [ ] 性能优化

---

## 当前状态
**正在执行**：阶段 1 - 项目基础设施搭建
**进度**：5%

---

## 已完成
- [✓] 项目分析和理解
- [✓] 创建重构计划文档

---

## 下一步行动
1. 与用户确认重构计划
2. 创建 Rust workspace 结构
3. 配置依赖项

---

## 技术决策

### Rust 依赖项选择
- **windows-rs**: 官方 Windows API 绑定
- **ntapi**: Windows NT API 绑定
- **winapi**: 传统 Windows API（如果需要）
- **pelite**: PE 文件解析（可选，或自己实现）
- **serde**: 序列化/反序列化
- **anyhow/thiserror**: 错误处理
- **log/env_logger**: 日志记录

### 架构设计
```
uacme-rust/
├── Cargo.toml (workspace)
├── akagi/          # 主程序
│   ├── Cargo.toml
│   └── src/
├── fubuki/         # 通用 payload
│   ├── Cargo.toml
│   └── src/
├── akatsuki/       # WOW64 logger payload
│   ├── Cargo.toml
│   └── src/
├── naka/           # 压缩工具
│   ├── Cargo.toml
│   └── src/
├── yuubari/        # UAC 信息转储
│   ├── Cargo.toml
│   └── src/
└── shared/         # 共享库
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── strings.rs
        ├── pe_loader.rs
        ├── cmdline.rs
        ├── winapi_ext.rs
        └── anti_emulator.rs
```

---

## 风险点

### 1. Windows API 兼容性
- **风险**：Rust 的 Windows API 绑定可能不完整
- **应对措施**：使用 `windows-rs` + 手动 FFI 绑定缺失的 API

### 2. PE 加载器实现
- **风险**：PE 加载器涉及底层内存操作，Rust 需要大量 unsafe 代码
- **应对措施**：仔细封装 unsafe 代码，确保内存安全

### 3. DLL 入口点
- **风险**：Rust 创建 DLL 的方式与 C 不同
- **应对措施**：使用 `#[no_mangle]` 和正确的调用约定

### 4. 32位/64位支持
- **风险**：需要同时支持 x86 和 x64
- **应对措施**：使用条件编译和交叉编译

### 5. 方法兼容性
- **风险**：某些方法可能依赖 C 特定的行为
- **应对措施**：逐个方法测试验证

---

## 注意事项

1. **保持原有功能**：确保所有有效的 UAC 绕过方法在 Rust 版本中正常工作
2. **安全性**：虽然是安全研究工具，但代码本身应该是内存安全的
3. **性能**：Rust 版本应该至少与 C 版本性能相当
4. **可维护性**：利用 Rust 的类型系统和模块化提高代码质量
5. **教育目的**：保持代码的可读性和教育价值

---

## 预计总时间
约 17 小时（分多个工作会话完成）

