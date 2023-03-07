#!/bin/sh

echo "[INFO] Install J..."

# Get script directory
SCRIPT_DIR=$(cd $(dirname $0); pwd)

# Build tool
echo "[INFO] Build J tool..."
pushd "$SCRIPT_DIR/J-bin"
cargo build --release
popd

echo "[INFO] Create ~/.lumi_j directory..."
mkdir -p ~/.lumi_j/bin
echo "[INFO] Copy J..."
cp "$SCRIPT_DIR/J" ~/.lumi_j/J
echo "[INFO] Copy J tool..."
cp "$SCRIPT_DIR/J-bin/target/release/_J_tool" ~/.lumi_j/bin/_J_tool

# Add to PATH
echo "[INFO] Add path and sources into ~/.xshrc..."
for file in ~/.bashrc ~/.zshrc ~/.ashrc; do
    if [ -f "$file" ]; then
        echo "" >> "$file"
        echo "#-v Added by lumiknit/J installer" >> "$file"
        echo "#   If you want to remove J, remove these lines." >> "$file"
        echo "export PATH=\$PATH:~/.lumi_j/bin" >> "$file"
        echo "source ~/.lumi_j/J" >> "$file"
        echo "#-^ Added by lumiknit/J installer" >> "$file"
    fi
done
