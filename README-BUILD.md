# Rudis 构建指南

## 构建脚本说明

本项目提供了 PowerShell 脚本 `build-release.ps1` 来自动化构建 Windows 和 Linux 平台的发行包，并为每个平台创建对应的 ZIP 压缩包。

## 使用方法

在项目根目录下运行以下命令：

```powershell
powershell -ExecutionPolicy Bypass -File build-release.ps1
```

## 构建产物

构建完成后，所有发行包文件将位于 `release` 目录中，按照版本号和平台分类：

- `rudis-0.1.0-windows-x86_64-msvc/`
  - `rudis-server.exe` - Windows MSVC 版本
  - `rudis.conf` - 配置文件
- `rudis-0.1.0-windows-x86_64-msvc.zip` - Windows MSVC 版本压缩包

- `rudis-0.1.0-linux-x86_64-musl/`
  - `rudis-server` - Linux 版本
  - `rudis.conf` - 配置文件
- `rudis-0.1.0-linux-x86_64-musl.zip` - Linux 版本压缩包

## 平台说明

### Windows 版本

项目支持两种 Windows 构建方式：
1. MSVC 版本（推荐）- 使用 Visual Studio 工具链
2. GNU 版本（可选）- 需要 MinGW-w64 工具链

### Linux 版本

使用 musl 工具链构建的静态链接版本，可以在大多数 Linux 发行版上运行。

## 故障排除

### GNU 构建失败

如果遇到 GNU 版本构建失败的错误：
```
error: Error calling dlltool 'dlltool.exe': program not found
```

这是因为系统缺少 MinGW-w64 工具链。如果不需要 GNU 版本，可以忽略此错误。

如需构建 GNU 版本，请安装 MinGW-w64 工具链。