# 安装 rust

我这里用的 WSL Ubuntu

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 安装完成以后检查是否安装成功
rustc --version

# 如果没有安装成功，可以重启一下终端。
```

## 更新和卸载

```bash
# 更新
rustup update

# 卸载
rustup self uninstall
```

# Rust 初探
 