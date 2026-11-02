# Attribution — RapidTally Ace

## AI assistance (LLM)

RapidTally Ace v0.1.0 — including the Rust workspace (`rapidtally-format`,
`rapidtally-cli`), the clean-room and platform documentation, the CI/CD
pipelines and the multi-platform release engineering — was produced with the
assistance of **GLM-5.3-Flash**, a large language model developed by **Z.ai**,
working under the direction of the project owner.

All AI-generated output in this repository is released under the project
license (MIT OR Apache-2.0) exactly like the rest of the code, and is subject
to the same clean-room protocol (`docs/CLEANROOM.md`): the model was given no
Intuit source code, SDK internals or decompiled material, and produced the
format heuristics from publicly available documentation and black-box
reasoning only.

## Logo

The RapidTally Ace logo (teal/orange retro "R" mark) was provided by the
project owner and is used with permission. © the project owner; all rights
reserved. Project image assets in `assets/` are derived from it (icons,
banner).

## Prior-art references (no code copied)

- **OpenQBW** (Apache-2.0) — an independent pure-Rust reader for QuickBooks
  Desktop files, referenced during feasibility research as proof that a
  clean-room read path is achievable. RapidTally Ace implements its own
  heuristics from scratch; no OpenQBW code is included. If future phases ever
  vendor its code, it will be kept under its Apache-2.0 license with NOTICE
  intact.
- **SAP SQL Anywhere public documentation** — for the general page-structured
  database model.

## Trademarks

QuickBooks is a registered trademark of Intuit Inc. RapidTally Ace is an
independent project and is not affiliated with, endorsed by, or sponsored by
Intuit Inc.

## Human direction

Product decisions, the QuickBooks compatibility goal, the platform matrix and
the clean-room policy are owned and directed by the project owner
(@maniesh-NR).
