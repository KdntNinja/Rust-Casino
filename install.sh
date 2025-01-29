# Install Rust using rustup in non-interactive mode
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install additional Rust components: rust-analyzer and rls
rustup component add rust-analyzer
rustup component add rls

# Add Cargo bin directory to the PATH for the current session and future sessions
echo "export PATH=\$HOME/.cargo/bin:\$PATH" >> ~/.bashrc

# Source the .bashrc file to update the PATH in the current session
source ~/.bashrc

# Verify installation
rustc --version
rustup show

# Success message
echo "Rust, rust-analyzer, and rls have been installed and the PATH has been updated."
