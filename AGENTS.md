# AGENTS.md

## Rust CLI Tool with msedge-tts and clap

- Use `cargo run -- <input>` to test CLI functionality
- msedge-tts requires Windows or WSL with Microsoft Edge installed
- Input can be from file (`--file path`) or stdin (no flag)
- Output is always MP3 format
- Add `clap` dependency with `cargo add clap`
- Add `msedge-tts` dependency with `cargo add msedge-tts`
- No test suite exists yet — run with `cargo run` to verify
- Always validate msedge-tts works with `echo "test" | cargo run`
- Never use `cargo build` without testing first — binary is minimal and untested
- This is a single-binary CLI — no config files or external dependencies beyond msedge-tts
- Do not add database or file system complexity — keep it simple
- Do not use async unless necessary — sync I/O is sufficient for CLI
- Do not add logging or verbose output unless user requests it
- Do not generate README unless explicitly requested
- Do not create tests until functionality is verified with real audio output

## Developer Commands

- `cargo run -- --file input.txt` — convert file to audio
- `echo "hello" | cargo run` — convert stdin to audio
- `cargo add clap` — add command-line parser
- `cargo add msedge-tts` — add text-to-speech engine
- `cargo run` — verify basic execution

## Constraints

- Must work on Windows/WSL
- Must output MP3
- Must accept file or stdin
- Must be single binary
- No external services
- No UI
- No config files
- No logging by default

## Architecture Notes

- Entry point: `src/main.rs`
- Use `clap` for args: `--file` and optional stdin
- Use `msedge-tts` to generate MP3
- Write output to stdout or `--output` flag (if added later)
- No state, no caching, no persistence
- No error recovery — fail fast on missing Edge or TTS

## Verification

1. `cargo add clap && cargo add msedge-tts`
2. `echo "test" | cargo run`
3. Listen for audio output
4. If silent, check Edge installation
5. If error, check `msedge-tts` docs

## Warnings

- Do not assume Linux native TTS — msedge-tts is Windows/WSL only
- Do not use `std::process::Command` for TTS — use `msedge-tts` crate
- Do not write files unless `--output` is specified
- Do not add dependencies beyond `clap` and `msedge-tts`
- Do not over-engineer — this is a simple CLI

## Future Improvements (Do Not Implement Yet)

- Output format selection (WAV, FLAC)
- Volume control
- Voice selection
- Language selection
- Progress indicators
- File output

> This file is auto-generated and should be updated when dependencies or behavior change.
