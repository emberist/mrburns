SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

if ! command -v cargo &> /dev/null
then
    echo "Cargo not found. Install it first: https://www.rust-lang.org/tools/install"

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

echo "Rust and cargo are installed 🔥 Let's install the project \n"

cargo install --path "$SCRIPTPATH"

echo "\n"
echo "Done. mrburns is installed. 🎉\n"
echo "Run mrburns --help to see the available commands.\n"
