
# 安装 anchor

```bash
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev cargo

# sudo apt install -y cargo
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force



echo "PATH=$PATH:/home/codespace/.cargo/bin"  >>~/.profile
echo "PATH=$PATH:/home/codespace/.cargo/bin"  >>~/.zshrc



avm install latest
avm use latest

```