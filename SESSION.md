# Session State

**Current Phase**: Complete
**Current Stage**: Draft PR published
**Last Checkpoint**: `f7fc159` - Complete Traditional Chinese client localization
**Planning Doc**: `docs/LOCALIZATION_PLAN.md`
**Draft PR**: https://github.com/GTanger/smudgy/pull/1

## Scope Guard

The fork now provides a maintainable `zh-TW` client, embedded CJK fonts, and
per-server UTF-8/Big5 compatibility without translating remote MUD/player or
third-party package content, transcoding Telnet/GMCP/MSDP protocol frames, or
forking Smudgy's hosted backend.

## Phase 1: Baseline and inventory ✅

- [x] Create `GTanger/smudgy`, configure `origin` / `upstream`, and branch from
  upstream commit `a282000eea17440df511c74b42df4afe6b889444`
- [x] Lock scope, non-scope, load-bearing relationships, and exit gates
- [x] Inventory first-party visible strings and trace locale, font, input,
  Telnet, VT, cloud-error, and wire-encoding boundaries
- [x] Record architecture and inventory in `docs/LOCALIZATION_PLAN.md` and
  `docs/LOCALIZATION_INVENTORY.md`

## Phase 2: Full client localization ✅

- [x] Add the `smudgy_i18n` Fluent catalog with English fallback
- [x] Persist and activate `system` / `en-US` / `zh-TW`
- [x] Localize first-party app shell, settings, connection flows, social/cloud
  feedback, automations, mapper, validation, and runtime messages
- [x] Preserve MUD output, player input, package names, author text, source
  code, and third-party descriptions verbatim
- [x] Add catalog-key parity, variable parity, missing-key, and visible-literal
  gates

## Phase 3: CJK and legacy encoding ✅

- [x] Bundle and license Noto Sans TC and Noto Sans Mono CJK TC
- [x] Verify embedded family metadata and representative Traditional Chinese
  glyph coverage
- [x] Persist per-server UTF-8 / Big5 with UTF-8 migration default
- [x] Stream-decode after Telnet framing and before VT parsing
- [x] Encode outbound semantic text while preserving raw protocol bytes
- [x] Cover split Big5 sequences, malformed input, ANSI, unmappable output,
  server persistence, and raw-frame invariants

## Phase 4: Product verification and publication ✅

### Automated evidence

- `cargo test -p smudgy_i18n`: 9 passed
- `cargo test -p smudgy_ui`: 147 passed
- `cargo test -p smudgy_widgets`: 12 passed
- `cargo test -p smudgy_core naming`: 8 passed
- `cargo test -p smudgy_core big5`: 4 passed
- outbound text vs raw protocol encoding test: 1 passed
- `cargo test --workspace --all-features --no-run`: all targets compiled
- `cargo build -p smudgy_ui --bin smudgy`: passed
- `git diff --check`: passed

### Visual acceptance

The rebuilt binary was opened and inspected directly in `zh-TW` with isolated
data. Accepted surfaces include locale selection, embedded CJK fonts, account
and security states, server/profile forms, UTF-8 and Big5 live loopback
sessions, ANSI text, CJK input and clipboard, automations, alias editor, map
editor, localized validation, responsive map scope controls, and the
`中斷連線` / `重新連線` state actions.

### Publication

- Code branch pushed to `GTanger/smudgy:agent/zh-tw-localization`
- Draft PR opened: https://github.com/GTanger/smudgy/pull/1

## Baseline limitations

- The upstream workspace has pre-existing broad rustfmt/clippy drift outside
  this change.
- A full runtime test pass can block on the existing Secret Service/keyring
  dependency in a headless session; all workspace/all-feature test targets
  compile successfully.

## Residue accounting

- Removed the isolated visual-QA data, map state, and screenshots.
- Stopped the test Smudgy process and UTF-8/Big5 loopback listeners.
- Removed temporary `xdotool` and `xclip` packages installed for visual QA;
  did not autoremove pre-existing dependencies.
- Retained the managed Cargo target cache at
  `/home/tanger/.cache/tanger/cargo-target` for normal reuse.
- Retained the intentional long-lived checkout at
  `/home/tanger/Projects/smudgy-zh-TW`; no repo-local `target/` exists.
