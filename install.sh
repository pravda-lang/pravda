cargo build --release
cp target/release/pravda .
echo "[install.sh]: adding stdlib..."
git clone https://github.com/pravda-lang/stdlib.git $HOME/stdlib

