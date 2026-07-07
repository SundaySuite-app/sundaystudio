# Distribution & auto-update

SundayStudio ships as installers for macOS and Windows via GitHub Releases. The
release pipeline (`.github/workflows/release.yml`) builds on tag; making it
**signed + auto-updating** is a matter of **secrets, keys and accounts only you
can provide**, plus a small amount of one-time wiring. This doc is the checklist.

SundayStudio is the least distribution-ready app in the suite: today's builds are
**unsigned** and there is **no auto-updater** yet. The Rust engine itself is
solid (audio-thread RT-safety, AI timeouts, no production panics — see the
Fase-2 soliditet review); this is purely a shipping-readiness gap.

## How it works

1. You bump the version (in `package.json`, `src-tauri/tauri.conf.json`, and
   `src-tauri/Cargo.toml` — keep all three in sync; currently `0.3.1`) and push
   a tag `vX.Y.Z`.
2. `release.yml` builds on macOS (Apple Silicon) and Windows, fetches the
   ffmpeg/ffprobe sidecars, and creates a **draft** GitHub Release with the
   installers attached. Signing/notarization/updater activate once the secrets +
   wiring below are in place.
3. You review the draft and publish it.

> **Deploy gotcha (same as the Electron apps):** the build uploads as a
> **draft**. "Publishing" is a separate manual step — review the draft, then mark
> it published/latest. A built-but-unpublished release is served to no one.

## Phase status

| Capability                   | State                                                                   |
| ---------------------------- | ----------------------------------------------------------------------- |
| Build macOS + Windows on tag | ✅ wired (`release.yml`)                                                |
| macOS signing + notarization | ❌ not wired — add the `bundle.macOS` block + Apple secrets below       |
| Windows signing              | ⏳ deferred (unsigned installer works; SmartScreen warns)               |
| Auto-update (`latest.json`)  | ❌ not wired — add `tauri-plugin-updater` + keypair + `plugins.updater` |

Until the Apple secrets are added, the workflow produces **unsigned** installers
(tauri-action skips signing when the secrets are absent). Unsigned apps warn on
first launch: macOS → right-click ▸ Open; Windows → "More info" ▸ "Run anyway".

## ⚠️ ffmpeg licensing — decide before ANY public release

SundayStudio bundles ffmpeg (used by `src-tauri/src/commands/export.rs`). The
common `ffmpeg-static` distribution is **GPL-3.0-or-later**, which is
**incompatible** with shipping SundayStudio as a proprietary/closed product
without offering corresponding source for the whole combined work. Options
before a public release (owner decision — same issue as SundayEdit):

- Swap to an **LGPL-only** ffmpeg build (compiled without `--enable-gpl` / x264 /
  x265 — you lose those encoders), or build your own LGPL ffmpeg, **or**
- keep the app **private/internal** (GPL obligations trigger primarily on
  distribution).

Private test builds are fine as-is.

## One-time wiring to enable signing + auto-update

This is the small amount of repo work needed before the secrets do anything. It
can be prepared now; nothing activates until the secrets/keypair exist.

1. **macOS bundle block** — add to `src-tauri/tauri.conf.json` under `bundle`:
   `"macOS": { "entitlements": "Entitlements.plist", "minimumSystemVersion": "10.15" }`,
   and add `src-tauri/Entitlements.plist` (microphone/audio-input entitlement —
   copy SundayRec's). See `../sundayrec/src-tauri`.
2. **Updater plugin** — `npm run tauri add updater`; register it in
   `src-tauri/src/lib.rs` (behind a `#[cfg(feature = "updater")]` if you want it
   optional), set `createUpdaterArtifacts: true`, and add
   `plugins.updater { pubkey: "<PUBLIC KEY>", endpoints:
["https://github.com/richardfossland/sundaystudio/releases/latest/download/latest.json"] }`.
3. **release.yml** — add the `TAURI_SIGNING_*` + `APPLE_*` env block (copy
   SundayRec's `tauri-action` step; it no-ops when secrets are absent), set
   `prerelease: false` (**important once the updater is wired**: the updater reads
   `/releases/latest`, which excludes pre-releases — a pre-release/draft means
   installed clients silently stop updating with a `latest.json` 404), and flip
   `includeUpdaterJson: true`.

## Required GitHub repository secrets

Settings → Secrets and variables → Actions → New repository secret.

### macOS code signing + notarization

Same Developer ID cert / credentials you already use for the other Sunday
desktop apps (team `784GN847G4`).

| Secret                       | Value                                                                                                                                                                         |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `APPLE_CERTIFICATE`          | Base64 of your "Developer ID Application" `.p12`: `base64 -i cert.p12 \| pbcopy`.                                                                                             |
| `APPLE_CERTIFICATE_PASSWORD` | The password set when exporting the `.p12`.                                                                                                                                   |
| `APPLE_SIGNING_IDENTITY`     | e.g. `Developer ID Application: Richard Fossland (784GN847G4)`. `security find-identity -v -p codesigning`.                                                                   |
| `APPLE_ID`                   | Your Apple Developer account email.                                                                                                                                           |
| `APPLE_PASSWORD`             | An **app-specific password** (appleid.apple.com → Sign-In and Security), not your account password. _(Note: the previously leaked one must be revoked/rotated — see MEMORY.)_ |
| `APPLE_TEAM_ID`              | `784GN847G4`.                                                                                                                                                                 |

### Auto-update signing

1. Generate the keypair (store the private key safely, **never** in the repo):
   ```bash
   npm run tauri signer generate -- -w ~/.tauri/sundaystudio_updater.key
   ```
2. Put the **public** key in `tauri.conf.json` under `plugins.updater.pubkey`.
3. Add these secrets:

| Secret                               | Value                                                                                                           |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| `TAURI_SIGNING_PRIVATE_KEY`          | Contents of `~/.tauri/sundaystudio_updater.key`: `cat ~/.tauri/sundaystudio_updater.key \| pbcopy`, then paste. |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | The password set for that key (empty string if generated without one).                                          |

> Keep the private key safe — losing it means existing installs can no longer
> auto-update (they'd need a manual reinstall with a new key).

### Windows code signing (deferred)

Not wired — the Windows installer is currently unsigned (works, SmartScreen
warns). An EV/OV code-signing cert + matching secrets is a later task.

## Cut a release

```bash
# bump version in package.json AND src-tauri/tauri.conf.json AND src-tauri/Cargo.toml
git tag v0.3.2
git push origin v0.3.2
# → watch the run, review the draft Release, then publish it.
```

## CI (every push to main / PR)

`.github/workflows/ci.yml` runs frontend lint/typecheck/tests and Rust
fmt/clippy/tests. No secrets required.
