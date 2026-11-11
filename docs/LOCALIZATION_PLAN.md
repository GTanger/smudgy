# Smudgy Traditional Chinese Localization Plan

## Outcome

Ship a maintainable Traditional Chinese (`zh-TW`) Smudgy client while keeping
the English upstream as the fallback and preserving the ability to merge future
changes from `smudgy-mud/smudgy`.

The localized client must remain a MUD client. It does not translate remote MUD
content and does not replace a MUD server or Smudgy's hosted cloud backend.

## Scope

- Add a real locale layer with English fallback and runtime locale selection.
- Localize every first-party user-visible UI string, validation message, status,
  dialog, window title, and client-generated error.
- Establish a reviewed `zh-TW` terminology glossary for MUD, mapper, scripting,
  automation, package, account, and social surfaces.
- Bundle or define a deterministic Traditional Chinese font fallback and verify
  CJK rendering, wrapping, clipping, input, clipboard, and terminal cell width.
- Add per-server text encoding selection for UTF-8 and Big5, with streaming
  inbound decoding and outbound encoding that does not corrupt Telnet control
  frames, GMCP, or MSDP.
- Preserve offline operation and explicitly identify hosted-cloud strings that
  cannot be controlled by this repository.
- Add automated coverage for locale fallback, message interpolation, config
  migration, UTF-8, Big5, split multibyte sequences, invalid bytes, and mixed
  Telnet/ANSI streams.
- Build and visually inspect the actual application in Traditional Chinese.

## Non-Scope

- Machine-translating text emitted by a connected MUD server.
- Forking or recreating Smudgy's hosted cloud backend or website.
- Rewriting the mapper, scripting runtime, package system, or UI design.
- Merging into the upstream repository without upstream maintainer review.

## Load-Bearing Relationships

1. **Upstream mergeability**: English remains authoritative fallback; translated
   text lives in locale resources instead of replacing source literals in place.
2. **Locale and settings**: the selected locale persists through existing
   settings without invalidating older settings files.
3. **Encoding and Telnet**: text transcoding occurs only on application text;
   IAC negotiation and protocol subnegotiations remain byte-exact.
4. **Encoding and profiles**: encoding is a server-level choice so all profiles
   connecting to the same MUD share the correct wire convention.
5. **Fonts and terminal layout**: glyph availability alone is insufficient;
   cell width, styled spans, links, selection, and input cursor positions must
   remain aligned for CJK text.
6. **Cloud boundary**: local UI errors are translated; arbitrary server-provided
   messages may remain source-language text and must not be misrepresented as a
   fully controlled surface.

## Phases and Exit Criteria

### Phase 1: Baseline and inventory

- Capture the upstream commit, workspace structure, current checks, and build
  prerequisites.
- Produce a complete first-party user-visible string inventory by subsystem.
- Trace settings, terminal input/output, Telnet, VT parsing, errors, and fonts.
- Select the locale-resource implementation from current primary documentation.

Exit: every translation/encoding/font surface has an owner and verification
route; baseline checks have a recorded result.

### Phase 2: Locale foundation

- Add English and `zh-TW` catalogs.
- Add locale persistence, fallback, lookup, interpolation, and tests.
- Convert all first-party UI and client-error call sites.
- Add a missing-key check so new English-only UI cannot silently enter later.

Exit: automated inventory reports no unclassified first-party user-visible
literal, and locale/fallback tests pass.

### Phase 3: CJK and legacy encoding

- Add deterministic Traditional Chinese font support.
- Add per-server UTF-8/Big5 settings with backwards-compatible defaults.
- Add streaming text conversion on both directions without touching protocol
  frames.
- Test split sequences, malformed input, ANSI styling, prompts, links, GMCP,
  MSDP, triggers, logs, and outgoing commands.

Exit: UTF-8 and Big5 fixtures round-trip correctly and CJK terminal positions
remain consistent.

### Phase 4: Product verification and publication

- Run formatting, lint, unit, integration, and release build checks.
- Launch the real client in `zh-TW`, inspect connection, preferences, mapper,
  automations/packages, account/social, error, and mixed-language states.
- Fix and re-inspect every visible clipping, untranslated first-party string,
  missing glyph, hierarchy, or layout failure.
- Commit, push, and open a draft pull request in the fork with evidence.

Exit: visual acceptance is `PASS`, automated checks pass or have explicit
environment-bound evidence, and retained assets/residue are accounted for.

## Verification Commands

The exact package-level commands will be refined after baseline discovery.
Expected gates include:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --release
```

## Source and Branch

- Upstream: `https://github.com/smudgy-mud/smudgy`
- Fork: `https://github.com/GTanger/smudgy`
- Development branch: `agent/zh-tw-localization`
- Starting commit: `a282000eea17440df511c74b42df4afe6b889444`
