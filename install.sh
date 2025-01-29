# Clear the terminal
clear

# Update package list and install rustc
sudo apt update
sudo apt install -y rustc

# Install rust-analyzer and rls using rustup (optional if needed)
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

sudo apt upgrade -y

echo "System updated"