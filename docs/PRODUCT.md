# the — product vision & layout

Moved verbatim from CLAUDE.md (2026-11-01) to keep the always-loaded agent context small.

## Product north star

```
SundayStudio is a podcast and jingle production application for churches.

Target user: a church volunteer or staff member who wants to record a
conversation podcast (sermon recap, ministry discussion, interview series)
and ship it without learning a real DAW. They have 1-8 microphones, a USB
interface, and 30 minutes.

Core promises:
1. Simpler than GarageBand. Truly. No instruments, no synthesis, no MIDI,
   no music creation surface. Just audio.
2. Powerful enough for serious podcast work: 8 simultaneous tracks, proper
   monitoring, broadcast-quality bundled effects.
3. AI does the boring audio engineering: leveling, noise removal, ducking,
   loudness normalization.
4. Jingle creation in under 60 seconds using AI music generation + smart
   voiceover templates.
5. Direct to podcast platform: LUFS-normalized export, RSS-ready, integrated
   with Spotify for Podcasters and Apple Podcasts.

Competitive positioning:
- vs GarageBand: podcast-first, not music-first; smaller project files;
  auto-mute and auto-duck built in
- vs Audacity: modern UI, AI features, much friendlier
- vs Hindenburg (~$95-300): one-time-cheap or subscription, AI-native
- vs Adobe Audition / Creative Cloud: a fraction of the price, focused,
  no learning curve
- vs Riverside / Squadcast: not remote-recording focused; we're for
  in-person church recording (sermons, panel discussions, Sunday school
  conversations)

Tech principles:
- Low-latency audio is non-negotiable. < 10ms monitor round-trip on common
  hardware.
- Recording reliability is sacred. A 90-minute recording must NEVER be
  lost to a crash. Continuous write to disk, watchdog process, recovery
  on restart.
- AI is opt-in per feature, with explicit consent the first time.
- Effects are bundled. NO third-party plugin support in v1 — that is what
  keeps us simpler than GarageBand. Plugin hosting is v2 if users demand.
- Sunday Pro tier unlocks AI, jingle generation, and direct platform upload.

Out of scope for v1 (most are revisit-after-launch):
- VST3/AU plugin hosting
- MIDI and software instruments
- Video recording (podcast video is a different problem; see SundayRec)
- Remote recording (Riverside-style)
- Loop-based music creation
```

