# Rudis Release Build Script
# 构建脚本会生成按版本号和平台分类的发行包，并创建对应的压缩包

# 从Cargo.toml获取版本号
function Get-VersionFromCargoToml {
    $cargoTomlPath = "Cargo.toml"
    if (Test-Path $cargoTomlPath) {
        $content = Get-Content $cargoTomlPath
        foreach ($line in $content) {
            if ($line -match 'version\s*=\s*"([^"]+)"') {
                return $matches[1]
            }
        }
    }
    return "unknown"
}

# 获取版本号
$version = Get-VersionFromCargoToml
Write-Host "检测到项目版本号: $version" -ForegroundColor Cyan

Write-Host "开始构建Rudis发行包..." -ForegroundColor Green

# 清理之前的构建产物
Write-Host "清理旧的构建产物..." -ForegroundColor Yellow
Remove-Item -Path "release/*" -Recurse -Force -ErrorAction SilentlyContinue

# 1. 构建Windows MSVC版本
Write-Host "1. 构建Windows MSVC版本..." -ForegroundColor Yellow
$cargoResult = cargo build --release --target x86_64-pc-windows-msvc
if ($LASTEXITCODE -ne 0) {
    Write-Host "Windows MSVC构建失败!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# 创建Windows MSVC目录结构并复制文件
$windowsDir = "release\rudis-$version-windows-x86_64-msvc"
New-Item -ItemType Directory -Path $windowsDir -Force | Out-Null
Copy-Item "target\x86_64-pc-windows-msvc\release\rudis-server.exe" "$windowsDir\"
Copy-Item "config\rudis.conf" "$windowsDir\"
Write-Host "Windows MSVC版本已复制到: $windowsDir" -ForegroundColor Green

# 2. 尝试构建Windows GNU版本（可选）
Write-Host "2. 尝试构建Windows GNU版本..." -ForegroundColor Yellow
$cargoResult = cargo build --release --target x86_64-pc-windows-gnu
$windowsGnuDir = $null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Windows GNU构建失败! 这不是必需的，继续构建其他版本..." -ForegroundColor Yellow
} else {
    # 创建Windows GNU目录结构并复制文件
    $windowsGnuDir = "release\rudis-$version-windows-x86_64-gnu"
    New-Item -ItemType Directory -Path $windowsGnuDir -Force | Out-Null
    Copy-Item "target\x86_64-pc-windows-gnu\release\rudis-server.exe" "$windowsGnuDir\"
    Copy-Item "config\rudis.conf" "$windowsGnuDir\"
    Write-Host "Windows GNU版本已复制到: $windowsGnuDir" -ForegroundColor Green
}

# 3. 构建Linux版本
Write-Host "3. 构建Linux版本..." -ForegroundColor Yellow
$cargoResult = cargo build --release --target x86_64-unknown-linux-musl
if ($LASTEXITCODE -ne 0) {
    Write-Host "Linux构建失败!" -ForegroundColor Red
    exit $LASTEXITCODE
}

# 创建Linux目录结构并复制文件
$linuxDir = "release\rudis-$version-linux-x86_64-musl"
New-Item -ItemType Directory -Path $linuxDir -Force | Out-Null
Copy-Item "target\x86_64-unknown-linux-musl\release\rudis-server" "$linuxDir\"
Copy-Item "config\rudis.conf" "$linuxDir\"
Write-Host "Linux版本已复制到: $linuxDir" -ForegroundColor Green

# 4. 创建压缩包
Write-Host "`n开始创建压缩包..." -ForegroundColor Green

# 压缩Windows MSVC版本
$windowsZip = "release\rudis-$version-windows-x86_64-msvc.zip"
Compress-Archive -Path $windowsDir -DestinationPath $windowsZip -Force
Write-Host "Windows MSVC版本压缩包已创建: $windowsZip" -ForegroundColor Green

# 如果Windows GNU版本存在，也创建压缩包
if ($windowsGnuDir -and (Test-Path $windowsGnuDir)) {
    $windowsGnuZip = "release\rudis-$version-windows-x86_64-gnu.zip"
    Compress-Archive -Path $windowsGnuDir -DestinationPath $windowsGnuZip -Force
    Write-Host "Windows GNU版本压缩包已创建: $windowsGnuZip" -ForegroundColor Green
}

# 压缩Linux版本
$linuxZip = "release\rudis-$version-linux-x86_64-musl.zip"
Compress-Archive -Path $linuxDir -DestinationPath $linuxZip -Force
Write-Host "Linux版本压缩包已创建: $linuxZip" -ForegroundColor Green

# 显示最终结果
Write-Host "`n构建完成! 发行包内容:" -ForegroundColor Green
Get-ChildItem -Path "release" -Directory | ForEach-Object {
    Write-Host "目录: $($_.Name)" -ForegroundColor Yellow
    Get-ChildItem -Path $_.FullName | ForEach-Object {
        Write-Host "  $($_.Name)" -ForegroundColor White
    }
}

Write-Host "`n压缩包列表:" -ForegroundColor Green
Get-ChildItem -Path "release" -Filter "*.zip" | ForEach-Object {
    Write-Host "压缩包: $($_.Name) (大小: $([math]::Round($_.Length / 1KB, 2)) KB)" -ForegroundColor Yellow
}

Write-Host "发行包和压缩包已生成完成" -ForegroundColor Green