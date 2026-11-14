#!/bin/bash
# Empacota o visor nativo para Linux.
# Uso (numa máquina Linux, ou na CI): ./scripts/bundle-linux.sh
# Saída:
#   dist/TsuroPDF-linux-{arch}/TsuroPDF
#   dist/TsuroPDF-{versão}-{triple}.tar.gz
#
# Sem .desktop e sem ícone de menu por enquanto: o pacote é um diretório com
# o binário e a `libpdfium.so` ao lado, que é onde `engine::bind` procura.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PDFIUM_RELEASE="chromium/8044"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/crates/tsuro/Cargo.toml" | head -1)"

case "$(uname -m)" in
  x86_64) ARCH="x86_64" ;;
  aarch64 | arm64) ARCH="arm64" ;;
  *)
    echo "arquitetura sem Pdfium publicado: $(uname -m)" >&2
    exit 1
    ;;
esac
TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"
STAGE="$ROOT/dist/TsuroPDF-linux-$ARCH"
TARBALL="$ROOT/dist/TsuroPDF-${VERSION}-${TRIPLE}.tar.gz"

# 1. Pdfium para dev e para o pacote (mesmo cache do bundle macOS).
LIB="$ROOT/libpdfium.so"
if [ ! -f "$LIB" ]; then
  ASSET="pdfium-linux-${ARCH}.tgz"
  echo "Baixando Pdfium ($ASSET)..."
  TMP="$(mktemp -d)"
  trap 'rm -rf "$TMP"' EXIT
  curl -fsSL --max-time 180 -o "$TMP/pdfium.tgz" \
    "https://github.com/bblanchon/pdfium-binaries/releases/download/${PDFIUM_RELEASE}/${ASSET}"
  tar xzf "$TMP/pdfium.tgz" -C "$TMP"
  cp "$TMP/lib/libpdfium.so" "$LIB"
  trap - EXIT
  rm -rf "$TMP"
fi

# 2. Binário release.
cargo build --release -p tsuro --manifest-path "$ROOT/Cargo.toml"

# 3. Pacote: binário + biblioteca + licença, lado a lado.
rm -rf "$STAGE" "$TARBALL"
mkdir -p "$STAGE"
cp "$ROOT/target/release/TsuroPDF" "$STAGE/TsuroPDF"
cp "$LIB" "$STAGE/libpdfium.so"
cp "$ROOT/LICENSE" "$STAGE/LICENSE"
tar czf "$TARBALL" -C "$ROOT/dist" "TsuroPDF-linux-$ARCH"

echo "Pronto: $STAGE"
echo "Pronto: $TARBALL"
