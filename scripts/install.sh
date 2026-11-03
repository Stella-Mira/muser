#!/bin/sh
# muser 一行安装 / 更新（macOS、Linux）：
#
#   curl -fsSL https://raw.githubusercontent.com/claw-works/muser/main/scripts/install.sh | sh
#
# 从 GitHub Releases 下载适合这台机器的发行包，校验 SHA256SUMS.txt，解到 ~/.local/share/muser/<版本>，
# 把 ~/.local/share/muser/current 指过去，并在 ~/.local/bin 放 muser、muser-mcp 两个链接。
# 再跑一次就是更新（`muser self-update` 也是调用它）；旧版本的目录留着，可手动删。
#
# 可选环境变量：
#   MUSER_VERSION=v0.4.0     装指定版本（默认最新）
#   MUSER_HOME=…             版本目录放在哪（默认 ~/.local/share/muser）
#   MUSER_BIN=…              链接放在哪（默认 ~/.local/bin）
#   MUSER_HEADLESS=1         Linux 上装不带声卡播放的版本（默认：没有 libasound.so.2 时自动选它）
#   MUSER_NO_MODIFY_PATH=1   不改 shell 配置文件（默认：~/.local/bin 不在 PATH 里时加一行）
set -eu

REPO="claw-works/muser"
HOME_DIR="${MUSER_HOME:-$HOME/.local/share/muser}"
BIN_DIR="${MUSER_BIN:-$HOME/.local/bin}"

say() { printf '%s\n' "$*"; }
die() { printf 'muser 安装失败：%s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || die "需要 $1，请先安装它"; }

need curl
need tar
need uname

# —— 这台机器用哪个发行包 ——
os=$(uname -s)
arch=$(uname -m)
case "$arch" in
  arm64 | aarch64) arch=aarch64 ;;
  x86_64 | amd64) arch=x86_64 ;;
  *) die "不支持的 CPU 架构：$arch" ;;
esac
variant=""
case "$os" in
  Darwin)
    # 在 Rosetta 里运行的终端会报 x86_64；Apple 芯片上装原生版本。
    if [ "$arch" = x86_64 ] && [ "$(sysctl -n hw.optional.arm64 2>/dev/null || echo 0)" = 1 ]; then arch=aarch64; fi
    target="$arch-apple-darwin"
    ;;
  Linux)
    target="$arch-unknown-linux-gnu"
    headless="${MUSER_HEADLESS:-auto}"
    if [ "$headless" = auto ]; then
      headless=1
      if ldconfig -p 2>/dev/null | grep -q 'libasound\.so\.2'; then headless=0; fi
      for d in /usr/lib /usr/lib64 /lib /lib64 /usr/lib/"$arch"-linux-gnu /lib/"$arch"-linux-gnu; do
        if [ -e "$d/libasound.so.2" ]; then headless=0; fi
      done
    fi
    if [ "$headless" = 1 ]; then variant="-headless"; fi
    ;;
  *) die "不支持的系统：${os}（Windows 请用 install.ps1）" ;;
esac

# —— 版本 ——
tag="${MUSER_VERSION:-}"
if [ -z "$tag" ]; then
  # releases/latest 会跳转到 …/releases/tag/<版本>：不调 API，不受频率限制。
  url=$(curl -fsSLo /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest") || die "连不上 GitHub"
  tag=${url##*/tag/}
  [ -n "$tag" ] && [ "$tag" != "$url" ] || die "没找到最新版本（${url}）"
fi
case "$tag" in v*) ;; *) tag="v$tag" ;; esac

name="muser-$tag-$target$variant"
base="https://github.com/$REPO/releases/download/$tag"
say "安装 muser ${tag}（$target${variant}）…"

tmp=$(mktemp -d 2>/dev/null || mktemp -d -t muser)
trap 'rm -rf "$tmp"' EXIT INT TERM
curl -fsSL -o "$tmp/$name.tar.gz" "$base/$name.tar.gz" || die "下载 $base/$name.tar.gz 失败"
curl -fsSL -o "$tmp/SHA256SUMS.txt" "$base/SHA256SUMS.txt" || die "下载 SHA256SUMS.txt 失败"

# —— 校验 ——
want=$(awk -v f="$name.tar.gz" '$2 == f || $2 == "*" f {print $1}' "$tmp/SHA256SUMS.txt")
[ -n "$want" ] || die "SHA256SUMS.txt 里没有 $name.tar.gz"
if command -v sha256sum >/dev/null 2>&1; then
  got=$(sha256sum "$tmp/$name.tar.gz" | awk '{print $1}')
else
  need shasum
  got=$(shasum -a 256 "$tmp/$name.tar.gz" | awk '{print $1}')
fi
[ "$got" = "$want" ] || die "校验不通过（下载的文件可能不完整），请重试"

# —— 解包、切换 current、放链接 ——
tar xzf "$tmp/$name.tar.gz" -C "$tmp"
[ -x "$tmp/$name/muser" ] || die "发行包里没有 muser"
mkdir -p "$HOME_DIR" "$BIN_DIR"
rm -rf "$HOME_DIR/$tag.new"
mv "$tmp/$name" "$HOME_DIR/$tag.new"
rm -rf "$HOME_DIR/$tag"
mv "$HOME_DIR/$tag.new" "$HOME_DIR/$tag"
ln -sfn "$tag" "$HOME_DIR/current"
for b in muser muser-mcp; do ln -sf "$HOME_DIR/current/$b" "$BIN_DIR/$b"; done

# —— PATH ——
case ":$PATH:" in
  *":$BIN_DIR:"*) on_path=1 ;;
  *) on_path=0 ;;
esac
if [ "$on_path" = 0 ] && [ "${MUSER_NO_MODIFY_PATH:-0}" != 1 ]; then
  line="export PATH=\"$BIN_DIR:\$PATH\""
  case "${SHELL:-}" in
    */zsh) rc="$HOME/.zshrc" ;;
    */bash) if [ "$os" = Darwin ]; then rc="$HOME/.bash_profile"; else rc="$HOME/.bashrc"; fi ;;
    *) rc="$HOME/.profile" ;;
  esac
  if ! grep -qsF "$line" "$rc"; then
    printf '\n# muser\n%s\n' "$line" >> "$rc"
    say "已把 $BIN_DIR 加进 PATH（写在 ${rc}，新开的终端生效）。"
  fi
fi

say ""
say "已安装：$("$HOME_DIR/current/muser" --version)  →  $HOME_DIR/$tag"
say "命令：$BIN_DIR/muser、$BIN_DIR/muser-mcp"
say "给 AI agent 的说明：$HOME_DIR/current/SKILLS.md"
if [ "$on_path" = 0 ]; then say "当前终端里先运行：export PATH=\"$BIN_DIR:\$PATH\""; fi
