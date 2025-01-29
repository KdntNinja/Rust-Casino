clear
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rust-analyzer
rustup component add rls
echo "export PATH=\$HOME/.cargo/bin:\$PATH" >> ~/.bashrc