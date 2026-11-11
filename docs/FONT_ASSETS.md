# Traditional Chinese Font Assets

Smudgy embeds two official Noto CJK 2.004 assets so Traditional Chinese
rendering does not depend on fonts installed on the host machine:

| Role | File | Family | Git blob SHA |
| --- | --- | --- | --- |
| Proportional application UI | `assets/fonts/NotoSansTC-Regular.otf` | Noto Sans TC | `63c3422382411b4d132644794fd432c53faa2882` |
| Monospaced terminal fallback and picker | `assets/fonts/NotoSansMonoCJKtc-Regular.otf` | Noto Sans Mono CJK TC | `5321e80fa9a339f07e9c0d09851db1bd183fd0b4` |

The files come from the official
[`notofonts/noto-cjk`](https://github.com/notofonts/noto-cjk) repository:

- `Sans/SubsetOTF/TC/NotoSansTC-Regular.otf`
- `Sans/Mono/NotoSansMonoCJKtc-Regular.otf`

Both are distributed under the SIL Open Font License 1.1. The exact upstream
license is retained as `assets/fonts/Noto-OFL.txt`; the application-visible
third-party notices already reproduce the same license text and now identify
both Noto families.

The blob hashes above were verified with `git hash-object` after download.
The embedded-family regression test in `ui/src/assets.rs` verifies the names
that Iced/cosmic-text must resolve.
