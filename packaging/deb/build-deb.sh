#!/usr/bin/env bash
# Build a Debian .deb package for RapidTally Ace from the static musl binary.
# Prereq: cargo build --release --target x86_64-unknown-linux-musl
# Output: dist/rapidtally-ace_${VERSION}_amd64.deb
set -euo pipefail

VERSION="${VERSION:-0.1.0}"
BIN="target/x86_64-unknown-linux-musl/release/rapidtally-ace"
PKG="packaging/deb/pkgroot/rapidtally-ace"

test -f "$BIN" || { echo "missing $BIN - build it first" >&2; exit 1; }

rm -rf packaging/deb/pkgroot
mkdir -p "$PKG/DEBIAN"
install -Dm755 "$BIN" "$PKG/usr/bin/rapidtally-ace"
install -Dm644 LICENSE-MIT "$PKG/usr/share/doc/rapidtally-ace/LICENSE-MIT"
install -Dm644 LICENSE-APACHE "$PKG/usr/share/doc/rapidtally-ace/LICENSE-APACHE"
install -Dm644 NOTICE.md "$PKG/usr/share/doc/rapidtally-ace/NOTICE.md"
install -Dm644 README.md "$PKG/usr/share/doc/rapidtally-ace/README.md"
install -Dm644 CHANGELOG.md "$PKG/usr/share/doc/rapidtally-ace/changelog"

cat > "$PKG/DEBIAN/control" <<EOF
Package: rapidtally-ace
Version: ${VERSION}
Section: office
Priority: optional
Architecture: amd64
Maintainer: RapidTally Ace Project <rapidtally-ace@users.noreply.github.com>
Installed-Size: $(du -sk "$PKG" | cut -f1)
Description: Clean-room QuickBooks Desktop database probe (RapidTally Ace, phase 0)
 RapidTally Ace is an independent, clean-room, cross-platform accounting
 suite. This package ships the phase-0 command-line structural probe for
 QuickBooks Desktop database files (.QBW/.QBB): ASCII marker scanning,
 candidate page-size analysis, entropy mapping and hex inspection.
 .
 Built with GLM (Z.ai) under the direction of the project owner.
 Not affiliated with Intuit Inc. "QuickBooks" is a trademark of Intuit Inc.
Homepage: https://github.com/maniesh-NR/RapidTally-Ace
EOF

mkdir -p dist
dpkg-deb --build --root-owner-group "$PKG" "dist/rapidtally-ace_${VERSION}_amd64.deb"
echo "built dist/rapidtally-ace_${VERSION}_amd64.deb"
