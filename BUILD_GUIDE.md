# Todo Assistant 构建指南

## 环境要求

### 必需的工具
1. **Node.js 20.x** - 前端开发
2. **Rust 1.70+** - 后端开发
3. **Visual Studio Build Tools** - Windows 构建（包含 MSVC 链接器）

## 构建选项

### 选项 1：使用 GitHub Actions（推荐）✅

代码推送到 GitHub 后会自动构建，无需本地配置。

**查看构建结果：**
1. 打开 https://github.com/evayuxinxia/Todolist/actions
2. 查看最新工作流运行状态
3. 构建成功后，在 **Artifacts** 下载安装包

### 选项 2：本地构建

#### 步骤 1：安装依赖

```powershell
# 安装 Rust（如果未安装）
winget install Rustlang.Rustup

# 安装 Visual Studio Build Tools
# 下载地址：https://visualstudio.microsoft.com/downloads/
# 安装时选择 "Desktop development with C++" 工作负载
```

#### 步骤 2：安装 npm 依赖

```powershell
cd d:\JavaProjects\todo_list
npm install
```

#### 步骤 3：验证前端构建

```powershell
npm run build
```

应该看到类似输出：
```
dist/index.html                          0.55 kB
dist/assets/main-*.css                   362.10 kB
dist/assets/main-*.js                    963.75 kB
built in 6.18s
```

#### 步骤 4：构建 Tauri 应用

```powershell
npm run tauri:build
```

构建完成后，安装包位于：
- `src-tauri/target/release/bundle/nsis/*.exe`
- `src-tauri/target/release/bundle/msi/*.msi`

## 常见问题

### 问题 1：link.exe not found

**原因：** 缺少 Visual Studio Build Tools

**解决方案：**
1. 下载 Visual Studio Build Tools：https://visualstudio.microsoft.com/downloads/
2. 安装时选择 **"Desktop development with C++"** 工作负载
3. 重启终端，重新运行构建

### 问题 2：npm install 失败

**解决方案：**
```powershell
# 清除缓存
npm cache clean --force

# 重新安装
rm -rf node_modules
npm install
```

### 问题 3：GitHub Actions 构建失败

**查看日志：**
1. 进入 https://github.com/evayuxinxia/Todolist/actions
2. 点击失败的工作流
3. 查看详细的错误日志

## 项目结构

```
todo_list/
├── src/                    # Vue 前端代码
├── src-tauri/             # Rust 后端代码
│   ├── src/
│   │   ├── lib.rs        # 主入口
│   │   ├── commands.rs   # Tauri 命令
│   │   ├── db.rs         # 数据库
│   │   ├── scheduler.rs  # 定时任务
│   │   └── ai_client.rs  # AI 客户端
│   └── target/           # 构建输出
│       └── release/bundle/  # 安装包
├── dist/                  # 前端构建输出
└── .github/workflows/    # CI/CD 配置
```

## 快速参考

| 命令 | 说明 |
|------|------|
| `npm run dev` | 开发模式运行 |
| `npm run build` | 构建前端 |
| `npm run tauri:dev` | 开发模式运行 Tauri |
| `npm run tauri:build` | 构建 Tauri 应用 |
