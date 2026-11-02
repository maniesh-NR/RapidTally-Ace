# Clean-room engineering protocol — RapidTally Ace

**Status:** mandatory for every contributor. **Version:** 1.0 (2026-11-02).

RapidTally Ace is engineered **clean-room**: the project's QuickBooks-related
behavior is derived exclusively from black-box observation and publicly
available documentation, never from Intuit source code, decompiled binaries,
SDK internals or any leaked material.

## 1. Definitions

- **Tainted material** — anything that could transmit Intuit copyright:
  QuickBooks source code, decompiled/disassembled QuickBooks binaries, SDK
  internals not publicly documented, leaked specifications, screenshots of
  internals, or analysis produced by studying such material.
- **Untainted material** — publicly available documentation, standards,
  black-box observations of files the project owner legally owns, and
  open-licensed third-party work (with attribution).
- **Spec team** — produces written specifications from untainted material only.
- **Implementation team** — writes code from the written specifications and
  untainted material only.

## 2. Team separation

1. The spec and implementation functions are kept separate. In this
   AI-assisted project the same discipline applies to *material channels*:
   research notes ("what the format looks like") and implementation inputs
   ("what to build") are separate, dated documents; implementation tasks cite
   specs, not raw research.
2. Any person (or AI session) that has been exposed to tainted material is
   **tainted** and must not contribute to the implementation until counsel
   approves a re-entry plan.
3. Every AI working session is logged in `docs/evidence/` with its inputs
   summarized, so an auditor can reconstruct what each unit of code was
   derived from.

## 3. Prohibited sources

- Intuit QuickBooks source code or object code (except running the software
  for black-box observation of *file* behavior, if legally licensed).
- Decompilation or disassembly of QuickBooks binaries — including feeding
  binaries to decompilers or AI tools.
- Non-public SDK materials, internal Intuit documentation, or leaked files.
- Copying from other clean-room-adjacent codebases whose license does not
  permit it.

## 4. Permitted sources

- Public SAP SQL Anywhere documentation and public research about the on-disk
  format.
- Black-box analysis of QuickBooks data files the project owner legally owns
  (after gate **G0**, see below), recorded as dated observations.
- The OpenQBW project (Apache-2.0) as *prior-art reading* to confirm
  feasibility. No code is copied; if code ever is reused, it will be vendored
  with its license and NOTICE intact.

## 5. Evidence log

All format observations go to `docs/evidence/` as append-only, dated entries:
sample hash, observation, hypothesis, and how the hypothesis was (in)validated.
The evidence log is the backbone of the abstraction-filtration defense
(Computer Associates v. Altai) — it shows what was independently created.

## 6. Legal frame (summary, not legal advice)

- **Sega v. Accolade / Atari v. Nintendo** (US): intermediate copying for
  black-box analysis of an interoperable format can be fair use.
- **Oracle v. Google** (US): reimplementation of APIs/structures needed for
  interoperability analyzed under fair use.
- **EU Directive 2009/24/EC** art. 5–6: decompilation for interoperability is
  permitted under conditions; observe-and-analyze is allowed for permitted use.
- **DMCA §1201(f)** (US): reverse engineering for interoperability exemption.
- Decision gates: **G0** (legal counsel memorandum) must be *approved* before
  any experiment on real `.QBW` files beyond trivial header sniffing.

## 7. Data hygiene

- **Never commit real `.QBW`/`.QBB` files** or any real customer accounting
  data to the repository, CI logs, issues or screenshots.
- All test fixtures are synthetic and generated in-code.
- If a real file is accidentally committed, treat it as a security incident
  (see `SECURITY.md`): purge history and rotate any exposed material.

## 8. Incident handling

If tainted material enters the repository or an implementation session:

1. Freeze the affected code and mark it quarantined.
2. Open a dated incident entry in `docs/evidence/`.
3. Rewrite the affected unit from untainted specs only.
4. Escalate to counsel if the taint reached a release.
