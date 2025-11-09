set -e
INSTALL_DIR="/usr/local/bin"
BINARY="ftr"

echo "[*] Compiling project..."
cargo build --release

echo "[*] Copying binary to $INSTALL_DIR ..."
sudo cp "target/release/$BINARY" "$INSTALL_DIR/"

cd /

echo "[*] Instalación completada: ejecuta '$BINARY' para usarlo."
