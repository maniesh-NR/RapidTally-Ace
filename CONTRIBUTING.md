# Contributing to RapidTally Ace

Thanks for your interest! Two rules matter more than code style here.

## 1. Clean-room protocol (mandatory)

Read [`docs/CLEANROOM.md`](docs/CLEANROOM.md) **before** writing any code
that touches QuickBooks-related behavior. In short:

- No Intuit source code, decompilation, leaked SDK material — ever.
- No real customer data files in issues, PRs, fixtures or screenshots.
- Format observations go to `docs/evidence/` as dated, append-only entries.
- PRs that add format knowledge must cite the spec/evidence they derive from.

## 2. Development workflow

```bash
git clone https://github.com/maniesh-NR/RapidTally-Ace.git
cd RapidTallyAce
cargo test --workspace          # toolchain auto-pinned to 1.77.2 by rustup
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

- The Rust toolchain is pinned to **1.77.2** (Windows 7 support ceiling);
  do not raise it without a change to `rust-toolchain.toml` + CI + docs.
- Zero external dependencies is a phase-0 goal — avoid adding crates unless
  the maintainers agree.
- Keep CLI output ASCII-only (Windows 7 legacy consoles).
- Commits: short imperative subject; reference evidence/spec docs when the
  change encodes format knowledge.

## 3. Release process (maintainers)

1. Merge to `main` with green CI on all three OSes.
2. Tag `vX.Y.Z` (annotated) and push the tag — the release workflow builds
   every platform artifact and attaches them plus `SHA256SUMS.txt`.
3. Verify the release page lists all expected assets.

## 4. Licensing

By contributing you agree your work is licensed under AGPL-3.0-or-later and
complies with the clean-room protocol.
