# Session State

**Current Phase**: Phase 2 - Locale foundation
**Current Stage**: Implementation
**Last Checkpoint**: `43d0b5c` - Phase 1 architecture and inventory
**Planning Doc**: `docs/LOCALIZATION_PLAN.md`

## Scope Guard

Deliver a maintainable `zh-TW` client, CJK product support, and UTF-8/Big5
per-server compatibility without rewriting Smudgy's game-independent systems,
forking its hosted backend, or translating remote MUD content. Closeout requires
automated checks, actual UI inspection, upstream/fork state, and 5S accounting.

## Phase 1: Baseline and inventory ✅

**Type**: Infrastructure / UI / Integration
**Started**: 2026-07-17
**Spec**: `docs/LOCALIZATION_PLAN.md#phase-1-baseline-and-inventory`

**Progress**:

- [x] Create `GTanger/smudgy` fork and long-lived local checkout
- [x] Add `origin`/`upstream` remotes and development branch
- [x] Lock complete scope, non-scope, load-bearing relationships, and exit gates
- [x] Record baseline build/test result
- [x] Inventory first-party visible strings by subsystem
- [x] Trace settings, errors, fonts, input, Telnet, VT, and wire encoding
- [x] Select and document the locale-resource implementation
- [x] Verify Phase 1 exit criteria

**Exit Evidence**: `docs/LOCALIZATION_INVENTORY.md`

**Key Files**:

- `docs/LOCALIZATION_PLAN.md`
- `Cargo.toml`
- `ui/src/lib.rs`
- `ui/src/windows/settings_window.rs`
- `core/src/models/settings.rs`
- `core/src/models/server.rs`
- `core/src/session/connection.rs`
- `core/src/session/connection/telnet.rs`
- `core/src/session/connection/vt_processor.rs`
- `cloud/src/error.rs`

**Known Issues**:

- Upstream formatting baseline has pre-existing rustfmt drift.
- Full baseline tests block in a Secret Service-dependent core test when run
  from this headless context; all test targets compile successfully.

## Phase 2: Locale foundation 🔄

**Spec**: `docs/LOCALIZATION_PLAN.md#phase-2-locale-foundation`

**Progress**:

- [x] Add `smudgy_i18n` workspace crate and English fallback
- [x] Persist and activate `system` / `en-US` / `zh-TW`
- [x] Add catalog parity, fallback, interpolation, and migration tests
- [ ] Convert first-party UI and client feedback by subsystem ← **CURRENT**
- [ ] Add missing-string inventory gate

**Next Action**: Implement and test the locale crate and backwards-compatible
settings field, then convert the app shell, connection, and preferences before
the larger mapper and automation surfaces.

## Phase 3: CJK and legacy encoding ⏸️

**Spec**: `docs/LOCALIZATION_PLAN.md#phase-3-cjk-and-legacy-encoding`

## Phase 4: Product verification and publication ⏸️

**Spec**: `docs/LOCALIZATION_PLAN.md#phase-4-product-verification-and-publication`
