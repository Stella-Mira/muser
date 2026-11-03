# muser 一行安装 / 更新（Windows PowerShell）：
#
#   irm https://raw.githubusercontent.com/claw-works/muser/main/scripts/install.ps1 | iex
#
# 从 GitHub Releases 下载 Windows 发行包，校验 SHA256SUMS.txt，解到 %LOCALAPPDATA%\muser\<版本>，
# 把 %LOCALAPPDATA%\muser\current（目录联接）指过去，并把 current 加进用户的 PATH。
# 再跑一次就是更新（`muser self-update` 也是调用它）；旧版本的目录留着，可手动删。
#
# 可选环境变量：MUSER_VERSION=v0.4.0（装指定版本，默认最新）、MUSER_HOME（默认 %LOCALAPPDATA%\muser）、
# MUSER_NO_MODIFY_PATH=1（不改 PATH）。
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'   # 进度条会让 Invoke-WebRequest 慢很多

$Repo = 'claw-works/muser'
$HomeDir = if ($env:MUSER_HOME) { $env:MUSER_HOME } else { Join-Path $env:LOCALAPPDATA 'muser' }
$Target = 'x86_64-pc-windows-msvc'   # Windows on ARM 用 x64 仿真运行

$Tag = $env:MUSER_VERSION
if (-not $Tag) {
    $Tag = (Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest").tag_name
    if (-not $Tag) { throw 'muser 安装失败：没找到最新版本' }
}
if (-not $Tag.StartsWith('v')) { $Tag = "v$Tag" }

$Name = "muser-$Tag-$Target"
$Base = "https://github.com/$Repo/releases/download/$Tag"
Write-Host "安装 muser $Tag ($Target) …"

$Tmp = Join-Path ([IO.Path]::GetTempPath()) ("muser-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $Tmp | Out-Null
try {
    $Zip = Join-Path $Tmp "$Name.zip"
    Invoke-WebRequest "$Base/$Name.zip" -OutFile $Zip -UseBasicParsing
    $Sums = (Invoke-WebRequest "$Base/SHA256SUMS.txt" -UseBasicParsing).Content
    if ($Sums -is [byte[]]) { $Sums = [Text.Encoding]::UTF8.GetString($Sums) }
    $Want = ($Sums -split "`n" | Where-Object { $_ -match "^([0-9a-fA-F]{64})\s+\*?$([regex]::Escape("$Name.zip"))\s*$" } | ForEach-Object { $Matches[1] }) | Select-Object -First 1
    if (-not $Want) { throw "muser 安装失败：SHA256SUMS.txt 里没有 $Name.zip" }
    $Got = (Get-FileHash $Zip -Algorithm SHA256).Hash
    if ($Got.ToLower() -ne $Want.ToLower()) { throw 'muser 安装失败：校验不通过（下载的文件可能不完整），请重试' }

    Expand-Archive $Zip -DestinationPath $Tmp -Force
    if (-not (Test-Path (Join-Path $Tmp "$Name\muser.exe"))) { throw 'muser 安装失败：发行包里没有 muser.exe' }
    New-Item -ItemType Directory -Force -Path $HomeDir | Out-Null
    $Dest = Join-Path $HomeDir $Tag
    if (Test-Path $Dest) { Remove-Item $Dest -Recurse -Force }
    Move-Item (Join-Path $Tmp $Name) $Dest

    # current 是目录联接：正在运行的旧 muser.exe 不用被覆盖，换个指向就行。
    $Current = Join-Path $HomeDir 'current'
    if (Test-Path $Current) { cmd /c rmdir "$Current" | Out-Null }
    New-Item -ItemType Junction -Path $Current -Target $Dest | Out-Null
} finally {
    Remove-Item $Tmp -Recurse -Force -ErrorAction SilentlyContinue
}

if ($env:MUSER_NO_MODIFY_PATH -ne '1') {
    $UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $Parts = if ($UserPath) { $UserPath -split ';' } else { @() }
    if ($Parts -notcontains $Current) {
        [Environment]::SetEnvironmentVariable('Path', (@($Current) + $Parts | Where-Object { $_ }) -join ';', 'User')
        Write-Host "已把 $Current 加进用户 PATH（新开的终端生效）。"
    }
    if (($env:Path -split ';') -notcontains $Current) { $env:Path = "$Current;$env:Path" }
}

Write-Host ''
Write-Host "已安装：$(& (Join-Path $Current 'muser.exe') --version)  ->  $Dest"
Write-Host "给 AI agent 的说明：$(Join-Path $Current 'SKILLS.md')"
