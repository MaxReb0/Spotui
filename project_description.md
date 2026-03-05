# Spotui: Spotify TUI in Rust — Learning Roadmap

## Context

Build a Spotify TUI using ratatui as a learning project. The codebase starts as a blank slate (just `ratatui = "0.30.0"` in Cargo.toml). This is a structured checklist with ordering rationale, project structure ideas, and key concepts to understand at each stage.

---

## Suggested Dependency Stack

Add these to Cargo.toml incrementally as you reach each phase:

```toml
# Async runtime
tokio = { version = "1", features = ["full"] }

# Spotify API client
rspotify = { version = "0.13", features = ["client-reqwest", "env-file"] }

# Terminal backend for ratatui
crossterm = "0.28"

# Event handling / async streams
futures = "0.3"

# Config / secrets management
dotenvy = "0.15"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling (pick one)
anyhow = "1"          # simpler, good for apps
# thiserror = "1"     # better for libraries

# Waveform stretch goal
rodio = "0.19"        # audio playback
```

---

## Suggested Project Structure

```
src/
├── main.rs             # Entry point, wires everything together
├── app.rs              # Core App state struct, tick/event loop
├── ui/
│   ├── mod.rs          # ui() render function dispatcher
│   ├── layout.rs       # Overall layout splitting
│   ├── home.rs         # Home / now playing view
│   ├── search.rs       # Search view
│   ├── playlist.rs     # Playlist browser view
│   └── waveform.rs     # (stretch) Waveform widget
├── spotify/
│   ├── mod.rs          # Re-exports
│   ├── auth.rs         # OAuth2 PKCE flow
│   └── client.rs       # Wrapper around rspotify calls
├── event.rs            # Event enum + event handler/thread
└── config.rs           # App config, keybindings, env vars
```

---

## Phase-by-Phase Checklist

### Phase 0: Understand the Tools
Before writing any Spotify code, get comfortable with each piece.

- [ ] Read the ratatui book: https://ratatui.rs/introduction/
- [ ] Build the ratatui "hello world" counter example from scratch (their tutorial)
- [ ] Understand the ratatui rendering loop: `terminal.draw(|frame| ui(frame, &app))`
- [ ] Read crossterm docs — understand how to read key events
- [ ] Understand the difference between `StatefulWidget` and `Widget` in ratatui

**Key concept:** ratatui is *immediate mode* — you redraw everything every frame. There's no retained widget tree.

---

### Phase 1: Basic TUI Event Loop

Goal: A working TUI that handles keyboard input and quits cleanly.

- [x] Add `crossterm` to Cargo.toml
- [x] Set up the terminal: `enable_raw_mode()`, `EnterAlternateScreen`
- [x] Write a `restore_terminal()` cleanup function and call it on panic too
- [x] Create a basic `App` struct with a `should_quit: bool` field
- [x] Implement the main loop:
  - Poll for crossterm events
  - Handle `q` / `Ctrl+C` to quit
  - Call `terminal.draw(...)` each tick
- [x] Render a simple "Hello Spotui" centered block

**Milestone:** `cargo run` opens a TUI, shows text, `q` exits cleanly.

---

### Phase 2: Spotify Authentication

Goal: Get an OAuth access token stored locally.

- [ ] Read the Spotify Web API docs on Authorization Code with PKCE
  - https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow
- [ ] Create a Spotify app at https://developer.spotify.com/dashboard
  - Set redirect URI to `https://localhost:8888/callback`
- [x] Add `rspotify` to Cargo.toml (with `client-reqwest` feature)
- [x] Add `tokio` with `full` features
- [ ] Write `spotify/auth.rs`:
  - Build the auth URL and open it in the browser (`open` crate or `std::process::Command`)
  - Spin up a minimal local HTTP listener to catch the callback code
  - Exchange code for tokens
  - Store token to disk (JSON file in `~/.config/spotui/token.json`) for reuse
- [ ] Add `dotenvy` and a `.env` file for `CLIENT_ID` (never hardcode credentials)
- [ ] On startup: try to load cached token, refresh if expired, auth if none

**Key concept:** rspotify handles token refresh for you if you use its `Token` struct correctly.

**Milestone:** `cargo run` authenticates with Spotify and prints your display name.

---

### Phase 3: Async Architecture

Goal: Make API calls without blocking the UI thread.

- [ ] Understand why you need async: API calls take time, the render loop must stay responsive
- [ ] Learn the pattern: spawn a tokio task for API work, send results back via `mpsc` channel
- [ ] Design your `Event` enum:
  ```rust
  enum Event {
      Key(KeyEvent),
      Tick,
      SpotifyData(SpotifyEvent),
  }
  enum SpotifyEvent {
      NowPlaying(CurrentPlaybackContext),
      SearchResults(SearchResult),
      Playlists(Vec<SimplifiedPlaylist>),
      // etc.
  }
  ```
- [ ] Create `event.rs` with an event loop that sends both key events and ticks through one channel
- [ ] Your `App` struct now holds a `tx: Sender<Event>` for triggering API calls

**Milestone:** App ticks every 250ms without blocking; key events register instantly.

---

### Phase 4: Now Playing View

Goal: Show what's currently playing.

- [ ] Call `rspotify`'s `current_playback()` endpoint
- [ ] Store result in `App` state: track name, artist, album, progress, duration
- [ ] Render a "Now Playing" panel:
  - Track info in a `Block` with borders
  - A progress bar using ratatui's `Gauge` widget
  - Play/pause, skip with keybindings (call Spotify API on keypress)
- [ ] Poll the playback state every 5 seconds on a background tick

**Milestone:** TUI shows live now-playing info and you can pause/skip from the terminal.

---

### Phase 5: Navigation & Multiple Views

Goal: Switch between different screens.

- [ ] Define a `View` enum: `Home`, `Search`, `Playlists`, `TrackList`
- [ ] Store current view in `App`
- [ ] Map number keys or letter keys to switch views (e.g., `1` = home, `2` = search, `3` = playlists)
- [ ] Build a tab bar at the top showing current view
- [ ] Implement a consistent footer showing keybindings context

**Key concept:** ratatui's `Layout` with `Constraint::Percentage` or `Constraint::Length` for splitting the screen.

---

### Phase 6: Search

Goal: Search for tracks, albums, artists.

- [ ] Build a search input using ratatui — handle typing character by character (crossterm key events)
- [ ] Learn about ratatui cursor positioning for the input field
- [ ] On Enter: call `rspotify`'s `search()` endpoint (dispatch via mpsc)
- [ ] Display results in a scrollable `List` widget
- [ ] `j`/`k` or arrow keys to navigate results
- [ ] Enter on a result: play it or show its tracks

---

### Phase 7: Playlist Browser

Goal: View and browse your playlists.

- [ ] Fetch user playlists via `current_user_playlists()`
- [ ] Display in a left-panel `List`
- [ ] On selection: fetch playlist tracks via `playlist_items()`
- [ ] Show tracks in a right panel (two-column layout)
- [ ] Allow playing a track from playlist context (so the queue continues)

**Key concept:** Spotify "context" matters — playing a track from a playlist sends it with context so Next/Prev work correctly.

---

### Phase 8: Polish & UX

- [ ] Add vim-style keybindings throughout (`g`/`G` for top/bottom, `/` for search)
- [ ] Show a loading spinner (ratatui `Throbber` or manual ASCII) during API calls
- [ ] Handle errors gracefully — show an error bar at the bottom, don't crash
- [ ] Add a `config.rs` for user-configurable keybindings (TOML file)
- [ ] Persist UI state across sessions (last view, scroll position)
- [ ] Show album art as unicode braille/sixel if terminal supports it (look at `viuer` crate)

---

### Phase 9 (Stretch): Waveform Visualizer

Goal: Animate a waveform of the current audio.

- [ ] Understand that Spotify Web API does NOT give you audio PCM data
- [ ] Two approaches:
  1. Use Spotify's Audio Analysis endpoint — pre-analyzed beat/segment data, animate to that
  2. Use `spotifyd` (a local Spotify daemon) + intercept local audio stream via `rodio`/`cpal`
- [ ] For approach 1: fetch `audio_analysis()` for current track, animate bars to beat timestamps
- [ ] Render using ratatui's `BarChart` widget or custom `Canvas` with braille dots
- [ ] Sync animation to `current_playback()` progress_ms

---

## Key Learning Concepts by Phase

| Phase | Rust Concepts |
|-------|--------------|
| 0-1   | Traits, closures, ownership in UI callbacks |
| 2     | async/await, reqwest, environment variables |
| 3     | mpsc channels, tokio::spawn, Arc<Mutex<T>> |
| 4-5   | Enums as state machines, pattern matching |
| 6-7   | Iterators, Vec operations, Option chaining |
| 8-9   | Config deserialization with serde, error types |

---

## Existing Projects to Study (not copy)

- **spotify-tui** (github: Rigellute/spotify-tui) — the canonical Rust Spotify TUI. Read the source for architecture patterns, especially how they handle the event loop and async Spotify calls. Note: it's somewhat old (pre-ratatui, uses tui-rs).
- **ratatui examples** — https://github.com/ratatui/ratatui/tree/main/examples — study `user_input.rs`, `list.rs`, `tabs.rs`

---

## Verification / How to Test Each Phase

- **Phase 1:** `cargo run` opens TUI, `q` exits, no terminal corruption after exit
- **Phase 2:** Run with valid CLIENT_ID, browser opens, token saved to disk, reused on next run
- **Phase 3:** Press keys rapidly — UI never freezes; check with `cargo test` for event logic
- **Phase 4:** Play a song in Spotify, run the app, confirm track info appears and updates
- **Phase 5:** Press view-switch keys, confirm panels change
- **Phase 6:** Type a search query, confirm results populate
- **Phase 7:** Browse playlists, select one, see tracks
