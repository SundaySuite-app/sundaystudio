//! The multi-track recording engine (Phase 1.2).
//!
//! Architecture (see docs/ARCHITECTURE.md for the diagrams):
//! ```text
//!   cpal callback ──push──▶ per-channel rtrb rings ──drain──▶ writer thread ──▶ WAVs
//!        │                                                         │
//!        └── atomic peak meters ◀── UI polls ──────────────────────┘
//! ```
//!
//! Split by testability:
//! - `writer` — multi-track WAV writing with crash-safe incremental flush (tested)
//! - `meters` — block peak + atomic peak-hold meters (tested)
//! - `command` — lock-free UI→audio command queue (tested)
//! - `monitor` — the low-latency monitor mixer: mono mix of armed tracks with soft mute (Phase 1.3, tested)
//! - `session` — rings + writer thread + controller; the FULL pipeline, driven in tests by synthetic frames, no device (tested)
//! - `stream` — the cpal input stream; the only hardware-dependent piece, wired to `session` but NOT verified without a real device
//!
//! The live start/stop/status Tauri commands now own the `Stream` on a dedicated
//! thread (`StreamHandle`) and hold the `RecordController` in app state (see
//! `commands::audio`'s `RecorderControl`). The pipeline + persistence are tested
//! with synthetic frames (no device); only the cpal `Stream` itself defers its
//! validation to the Phase 2.2 on-device matrix.

pub mod command;
pub mod meters;
pub mod monitor;
pub mod rate;
pub mod resampler;
pub mod session;
pub mod stream;
pub mod writer;

pub use command::{CommandRx, CommandTx, RecorderCommand, command_channel};
pub use meters::PeakMeters;
pub use monitor::{MAX_MONITOR_TRACKS, MonitorState, mix_monitor_block};
pub use session::{CaptureSink, RecordConfig, RecordController, start_session};
pub use stream::{StreamHandle, build_capture_stream, find_input_device};
pub use writer::{MultiTrackWriter, TrackSpec};
