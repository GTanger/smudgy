# Smudgy Localization Inventory

This inventory classifies the localization-bearing surfaces at upstream commit
`a282000`. Counts are discovery aids, not promises that every string match is a
user-visible sentence: identifiers, protocol tokens, logs, tests, and user or
server content are deliberately separated below.

## Baseline

- Workspace metadata and all test targets compile after installing the Linux
  development packages `libdbus-1-dev`, `libgtk-3-dev`, and
  `libwebkit2gtk-4.1-dev`.
- `cargo test --workspace --all-features --no-run` passes.
- The full test run reaches the existing core test
  `missing_required_params_tracks_only_unset_required_keys`, then blocks while
  accessing the desktop Secret Service in this headless launch context. The run
  was stopped after the test exceeded three minutes. This predates localization
  changes and must be isolated from the fork's own tests.
- `cargo fmt --all -- --check` fails on the unchanged upstream tree with broad
  existing rustfmt drift. Fork changes must be formatted file-by-file so this
  unrelated churn is not folded into localization commits.
- Build artifacts use the managed cache
  `/home/tanger/.cache/tanger/cargo-target`; the project tree contains no
  `target` directory.

## Discovery Counts

The broad source scan found 767 candidate string-bearing calls in `ui`, 236 in
`core`, 91 in `cloud`, and 9 in `widgets`. A narrower scan found approximately
446 direct UI constructor calls. These overlap and include false positives;
the classification and ownership below determine what is translated.

The largest UI concentrations are:

| Surface | Candidate concentration | Owner / verification |
| --- | ---: | --- |
| Automation package browser and configuration | 198 | `ui/src/windows/automations_window/packages.rs`; package flow screenshot and interaction |
| Map editor dialogs | 103 | `ui/src/windows/map_editor_window/modals.rs`; create/edit/delete flows |
| Preferences | 76 | `ui/src/windows/settings_window.rs`; every settings tab and locale switch |
| Map editor inspector | 58 | `ui/src/windows/map_editor_window/inspector.rs`; room/exit/area selection states |
| Automation manifest | 49 | `ui/src/windows/automations_window/manifest.rs`; manifest editor validation |
| Automation editors | 48 | `ui/src/windows/automations_window/editors.rs`; alias/trigger/hotkey/timer flows |
| Social panel | 34 | `ui/src/components/social_panel.rs`; signed-in, signed-out, loading, empty, and error states |
| Connection profile/server dialogs | 48 combined | `ui/src/components/modal/connect.rs`; create/edit/connect/error and encoding selection |
| Automation store inspector | 23 | `ui/src/windows/automations_window/store_inspector.rs`; empty/data/error states |

## Included First-Party Surfaces

1. **Application shell** — window titles, toolbar actions, session headings,
   upgrade prompts, account state, modal buttons, tooltips, and empty states.
2. **Connection management** — server and profile fields, validation,
   passwords, connection lifecycle echoes, disconnect causes, and trusted-link
   prompts.
3. **Preferences** — section names, field labels, descriptions, enumerated
   choices, font/theme controls, logging, updates, scripting, and language.
4. **Terminal client feedback** — client-generated connection/runtime/package
   status and warning lines. Remote MUD prose is never passed through the locale
   catalog.
5. **Automations and packages** — aliases, triggers, hotkeys, timers, scripts,
   package install/update/publish/configuration, manifests, parameter labels,
   validation, confirmations, and errors.
6. **Mapper** — editor chrome, area/room/exit inspectors, dialogs, commands,
   selection state, sync state, local/cloud scope messages, and validation.
7. **Cloud/account/social** — locally controlled authentication, update,
   friend, presence, sync, and API error explanations. An arbitrary message
   supplied by the hosted service remains visibly attributable source text.
8. **Accessibility text** — labels, descriptions, placeholders, and tooltips
   that convey meaning even when not continuously visible.

## Explicitly Excluded or Preserved Verbatim

- Text emitted by a connected MUD server, other players, package authors, or
  user-authored scripts.
- Server names, profile names, package names, usernames, room names, paths,
  URLs, versions, regexes, command text, and other user/domain data.
- Telnet option names and bytes, GMCP/MSDP module and field names, ANSI/VT
  control data, JavaScript API identifiers, JSON keys, CSS names, key codes,
  and persisted wire tags.
- Developer-only log records, internal assertion text, test fixtures, code
  comments, licenses, and third-party notices.
- Familiar product/technology names such as Smudgy, MUD, GMCP, MSDP, UTF-8,
  Big5, JavaScript, JSON, URL, and API; surrounding prose is localized.

## Architecture Traces

### Locale and settings

`core/src/models/settings.rs` is the backwards-compatible persisted settings
owner. New locale data must carry a serde default so older files still parse.
`ui/src/lib.rs::init` loads settings before rendering; locale activation must
occur there before the first window opens. System detection must prefer
`LANGUAGE` before `LC_ALL`/`LC_MESSAGES`/`LANG` so a desktop configured as
`zh_TW` is not masked by a process-level `LC_ALL=C.UTF-8`.

### Errors and client feedback

UI errors are centralized partly in `ui/src/components/cloud_errors.rs`, while
session lifecycle and runtime echoes originate in `core/src/session.rs` and
`core/src/session/connection.rs`. Typed error variants should map to catalog
messages at the display boundary. Opaque hosted-service text must not be
relabeled as first-party translated prose.

### Fonts and CJK layout

Bundled Geist, Monaspace, Courier Prime, Departure Mono, Fira Mono, Lilex, and
VT323 do not provide deterministic Traditional Chinese coverage. The fork must
register a redistributable Traditional Chinese UI face and monospaced terminal
face, retain Latin font choices, and exercise double-width text through shaping,
wrapping, styled spans, selection, hyperlinks, clipboard, and the input cursor.

### Telnet, VT, logs, triggers, and wire text

The Telnet parser removes negotiations before `TelnetBridge::on_data` feeds
application bytes to the VT parser. A persistent streaming decoder belongs at
that boundary so Big5 code units split across socket reads remain intact.
Inbound raw logging retains original bytes; decoded UTF-8 then flows through VT
parsing, line generation, triggers, plain logs, links, and display. Outbound
encoding applies only to semantic text messages. Raw bytes, IAC negotiations,
and protocol subnegotiations remain byte-exact; GMCP JSON is not blindly
transcoded.

Encoding is server-level state, not profile-level state, because every profile
for one MUD shares the same wire convention. Existing server records default to
UTF-8.

## Locale Implementation Decision

Use a small independent workspace crate, `smudgy_i18n`, backed by Project
Fluent resources. It owns:

- the persisted locale preference (`system`, `en-US`, `zh-TW`);
- deterministic locale resolution and English fallback;
- embedded `en-US` and `zh-TW` catalogs;
- message interpolation and missing-message diagnostics;
- a shared active locale usable by both core-generated feedback and UI views.

Stable message identifiers keep English copy and Traditional Chinese wording
reviewable without source-literal matching. A catalog parity test and a source
inventory check prevent new unclassified first-party English UI from silently
shipping.

## Phase 1 Exit

Every identified surface now has an owner and a verification path; settings,
font, error, terminal, Telnet, VT, trigger, log, and wire relationships are
traced; baseline failures are separated from fork changes; and the locale
architecture is selected. Phase 2 begins with the independent locale crate and
settings migration.
