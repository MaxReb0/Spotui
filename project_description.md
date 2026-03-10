# Spotui: Spotify TUI in Rust — Learning Roadmap

## Context

Build a Spotify TUI using ratatui as a learning project. The codebase starts as a blank slate (just `ratatui = "0.30.0"` in Cargo.toml). This is a structured checklist with ordering rationale, project structure ideas, and key concepts to understand at each stage.

---

## Dependency Stack

```toml
# Async runtime
tokio = { version = "1", features = ["full"] }

# Spotify Web API client
rspotify = { version = "0.15", features = ["cli", "env-file"] }

# Terminal backend for ratatui
crossterm = { version = "0.29", features = ["event-stream"] }

# Async stream utilities
tokio-stream = "0.1"
futures = "0.3"

# Trait objects for async functions
async-trait = "0.1"

# Error handling
color-eyre = "0.6"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Config dir resolution
directories = "6"

# Audio playback (Phase 7)
librespot = "0.6"

# Serialization (Phase 9)
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

## Project Structure

```
src/
├── main.rs                 # Entry point, wires everything together
├── app.rs                  # Core App state struct, select! loop
├── ui/
│   ├── mod.rs              # View dispatch
│   ├── layout.rs           # Overall layout splitting
│   ├── home.rs             # Home view (stub until Phase 8)
│   ├── search.rs           # Search view
│   ├── playlist.rs         # Playlist browser view
│   └── now_playing.rs      # Now playing view (Phase 8)
├── spotify/
│   ├── mod.rs              # Re-exports
│   ├── auth.rs             # OAuth2 PKCE flow
│   ├── client.rs           # SpotifyClient + SpotifyApi trait
│   └── spotify_event.rs    # SpotifyEvent enum
├── librespot/
│   ├── mod.rs              # Re-exports
│   └── player.rs           # Librespot session + PlayerEvent handling
└── logging.rs              # Tracing setup
```

---

## Phase-by-Phase Checklist

### Phase 0: Understand the Tools
- [x] Read the ratatui book: https://ratatui.rs/introduction/
- [x] Build the ratatui "hello world" counter example from scratch
- [x] Understand the ratatui rendering loop: `terminal.draw(|frame| ui(frame, &app))`
- [x] Read crossterm docs — understand how to read key events
- [x] Understand the difference between `StatefulWidget` and `Widget` in ratatui

**Key concept:** ratatui is *immediate mode* — you redraw everything every frame.

---

### Phase 1: Basic TUI Event Loop

- [x] Add `crossterm` to Cargo.toml
- [x] Set up the terminal via `ratatui::init()` and `ratatui::restore()`
- [x] Panic cleanup via `color_eyre::install()`
- [x] Create a basic `App` struct with an `exit: bool` field
- [x] Implement the main loop — handle `q` / `Ctrl+C`, call `terminal.draw(...)`
- [x] Render a simple "Hello Spotui" centered block

**Milestone:** `cargo run` opens a TUI, shows text, `q` exits cleanly.

---

### Phase 2: Spotify Authentication

- [x] Create a Spotify app at https://developer.spotify.com/dashboard
- [x] Add `rspotify` to Cargo.toml (with `cli` and `env-file` features)
- [x] Write `spotify/auth.rs` — PKCE flow, browser prompt, token cached to disk
- [x] Use `directories` crate to resolve `~/.config/spotui/token.json`
- [x] On startup: load cached token → refresh if expired → full auth if none
- [x] Wrap `AuthCodePkceSpotify` in `SpotifyClient` struct
- [x] Display username in TUI on startup

**Milestone:** `cargo run` authenticates with Spotify and shows your display name.

---

### Phase 3: Async Architecture

- [x] Add `event-stream` feature to crossterm, add `futures` and `tokio-stream`
- [x] Make `App::run()` async
- [x] Replace blocking `event::read()` with `EventStream` + `tokio::time::interval`
- [x] Use `tokio::select!` with three branches: tick, crossterm events, `rx.recv()`
- [x] Define `SpotifyEvent` enum in `spotify/spotify_event.rs`
- [x] Define `SpotifyApi` trait with `#[async_trait]` — only methods `App` calls
- [x] `App` holds `Arc<dyn SpotifyApi>` — enables mock testing
- [x] Spawn one-off `tokio::task` for API calls, send result back via `tx`
- [x] Write unit tests: key event → state transitions, async event round-trip

**Key concept:** `tokio::select!` is the multiplexer. Rendering happens on the tick branch only.

**Milestone:** App redraws at fixed rate; key events register instantly; async Spotify calls don't block the UI.

---

### Phase 4: Navigation & Multiple Views

Goal: Build the structural shell that all future views live inside.

- [x] Define a `View` enum: `Home`, `Search`, `Playlists`
- [x] Store `current_view: View` in `App`
- [x] Map keys to switch views: `1` = Home, `2` = Search, `3` = Playlists
- [ ] Build a tab bar at the top showing the active view
- [ ] Build a footer showing context-sensitive keybindings
- [ ] Create stub render functions for each view in `ui/`
- [ ] Implement overall layout split: tab bar / content area / footer

**Key concept:** ratatui's `Layout` with `Constraint::Length` for fixed-height bars and `Constraint::Min` for the content area.

**Milestone:** Three views selectable by key; tab bar updates to reflect current view; each view shows a placeholder.

---

### Phase 5: Search

Goal: Search for tracks and queue them for playback.

- [ ] Add search input state to `App`: query string, results, selected index
- [ ] Render a text input field in the search view — handle character-by-character typing
- [ ] Learn about ratatui cursor positioning for the input field
- [ ] On `Enter`: spawn a task calling `rspotify`'s `search()` endpoint
- [ ] Add `SearchResults` variant to `SpotifyEvent`, update `rx.recv()` handler
- [ ] Display results in a scrollable `List` widget
- [ ] `j`/`k` or arrow keys to move selection up/down
- [ ] `Enter` on a result: queue it for playback (wired to librespot in Phase 7)

**Milestone:** Type a query, results populate; navigate with `j`/`k`.

---

### Phase 6: Playlist Browser

Goal: Browse your playlists and their tracks.

- [ ] Fetch user playlists via `current_user_playlists()` on view load
- [ ] Display playlists in a left-panel `List`
- [ ] On selection: fetch tracks via `playlist_items()`, show in right panel
- [ ] Two-column layout: playlist list | track list
- [ ] Add `Playlists` and `PlaylistTracks` variants to `SpotifyEvent`
- [ ] `Enter` on a track: queue for playback (wired to librespot in Phase 7)

**Key concept:** Spotify "context" matters — playing a track with playlist context means Next/Prev work correctly.

**Milestone:** Browse playlists, select one, see its tracks.

---

### Phase 7: Librespot Integration

Goal: Play audio directly from the app.

- [ ] Add `librespot` to Cargo.toml
- [ ] Create `librespot/player.rs` — set up a librespot session and audio player
- [ ] Understand librespot's auth vs rspotify's auth — two separate sessions, same account
- [ ] Wire up `PlayerEvent` channel from librespot into the `tokio::select!` loop
- [ ] When a track URI is selected (from search or playlist), send it to librespot to play
- [ ] Handle `PlayerEvent::Playing`, `PlayerEvent::Paused`, `PlayerEvent::EndOfTrack`
- [ ] Expose play/pause/skip controls via keybindings

**Key concept:** librespot is receive-only for Connect commands but can be driven directly via its API. rspotify handles search/browse; librespot handles audio.

**Milestone:** Select a track from search or playlist — it plays through your speakers.

---

### Phase 8: Now Playing View

Goal: Show real-time playback state, powered by librespot events.

- [ ] Add `NowPlaying` to the `View` enum, reachable via `4` key
- [ ] On `PlayerEvent::Playing { track_id, .. }`: fetch metadata via librespot's `Metadata` API
- [ ] Store in `App`: track name, artist, album, duration, position
- [ ] On `PlayerEvent::PositionChanged`: update progress in `App` state
- [ ] Render the Now Playing view:
  - Track info in a bordered `Block`
  - Progress bar using ratatui's `Gauge` widget
  - Play/pause, skip keybindings wired to librespot controls
- [ ] Replace temporary rspotify `current_playback()` with librespot event data

**Key concept:** librespot's `PositionChanged` events give low-latency progress updates without polling the Web API.

**Milestone:** Now Playing view shows live track info and updates in real time as songs change.

---

### Phase 9: Polish & UX

- [ ] Add vim-style keybindings throughout (`g`/`G` for top/bottom, `/` to jump to search)
- [ ] Show a loading spinner during API calls
- [ ] Handle errors gracefully — error bar at the bottom, no crashes
- [ ] Add a `config.rs` for user-configurable keybindings (TOML file)
- [ ] Persist UI state across sessions (last view, scroll position)
- [ ] Show album art as unicode braille/sixel if terminal supports it (`viuer` crate)

---

### Phase 10 (Stretch): Waveform Visualizer

Goal: Animate a waveform synced to the playing audio.

- [ ] Use librespot's `PlayerEvent` beat/segment data or Spotify's Audio Analysis endpoint
- [ ] Render using ratatui's `BarChart` or custom `Canvas` with braille dots
- [ ] Sync animation to `PlayerEvent::PositionChanged` timestamps

---

## Key Learning Concepts by Phase

| Phase | Rust Concepts |
|-------|--------------|
| 0-1   | Traits, closures, ownership in UI callbacks |
| 2     | async/await, reqwest, environment variables |
| 3     | mpsc channels, tokio::spawn, Arc<dyn Trait> |
| 4     | Enums as state machines, pattern matching, Layout |
| 5-6   | Iterators, Vec operations, Option chaining |
| 7     | FFI-adjacent crates, multi-session auth, event streams |
| 8     | Async event-driven state updates |
| 9     | Config deserialization with serde, error types |

---

## Verification / How to Test Each Phase

- **Phase 1:** `cargo run` opens TUI, `q` exits, no terminal corruption
- **Phase 2:** Browser opens, token saved to disk, reused on next run
- **Phase 3:** `cargo test` passes; keys never freeze the UI
- **Phase 4:** Press `1`/`2`/`3` — tab bar updates, stub views render
- **Phase 5:** Type a query, results appear, `j`/`k` navigates
- **Phase 6:** Browse playlists, select one, tracks appear in right panel
- **Phase 7:** Select a track — audio plays through speakers
- **Phase 8:** Now Playing view shows correct track, progress bar moves

---

## Existing Projects to Study (not copy)

- **spotify-tui** (github: Rigellute/spotify-tui) — canonical Rust Spotify TUI. Study event loop and async patterns. Note: pre-ratatui, uses tui-rs.
- **ratatui examples** — https://github.com/ratatui/ratatui/tree/main/examples — study `user_input.rs`, `list.rs`, `tabs.rs`
- **librespot examples** — https://github.com/librespot-org/librespot/tree/master/examples — study `play.rs` and `play_connect.rs`
