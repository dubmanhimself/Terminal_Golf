# Terminal Golf

A terminal-based golf game in Rust with colorful ASCII visuals.

## Requirements

- Rust toolchain (install with `rustup`): https://rustup.rs
- Any modern terminal (macOS Terminal/iTerm2, Windows Terminal, Linux terminal emulators)

## Run

```bash
cargo run
```

## Controls

- `A` / `D` or arrow keys left/right: aim (full 360)
- `W` / `S` or arrow keys up/down: cycle clubs
- `E`: cycle swing type (`Full`, `3/4`, `Half`, `Pitch`, `Chip`)
- `C`: toggle auto-caddie on/off
- `Space` or `Enter`: hit ball
- `R`: restart hole
- `Q` or `Esc`: quit

## Current Version

- Single playable hole
- Full club bag (Driver through wedges + putter)
- Manual club selection with optional auto-caddie
- Auto-caddie can select club and swing type by remaining distance
- Five non-putter swing types: `Full`, `3/4`, `Half`, `Pitch`, `Chip`
- Putter green behavior tuned for easier, more controllable putting
- Realistic yardage table mapped to arcade-friendly tile distances
- Ball flight arc for non-putter shots
- Surface-dependent physics (green/fairway/rough/bunker)
- Full-screen green zoom camera when on/near the green
- Putt direction/error HUD hints for easier green alignment
- Little golfer sprite appears at address before each shot
- Stroke + par tracking with yard distance in HUD

## Notes

- Swing type controls are ignored for the putter.
- You can fully rotate to recover after overshooting.
- Driver no longer auto-drops; cup capture is tighter and the green is offset to require aim.

## Next Steps

- Multiple holes loaded from map data
- Better camera transitions and shot animations
- Scorecard across 9/18 holes
- Lightweight sound effects for impact and cup sink
