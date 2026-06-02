# rustled

A fast, minimal CLI tool that converts text to MP3 audio using Microsoft Edge TTS.

## Features

- Convert text files or stdin to MP3
- Voice selection with partial matching
- Adjustable rate, pitch, and volume
- SSML input support
- Batch mode for multi-line processing
- List voices with optional locale filtering
- Output to file or stdout
- Debug output to stderr

## Requirements

- Windows or WSL with Microsoft Edge installed
- Rust (for building from source)

## Installation

### From crates.io

```bash
cargo install rustled
```

### From source

```bash
git clone https://github.com/atareao/rustled
cd rustled
cargo build --release
```

## Usage

```bash
# Convert text from stdin
echo "Hello, world!" | rustled

# Convert a file
rustled --file input.txt

# Specify output file
rustled --file input.txt --output audio.mp3

# Output to stdout (for piping)
rustled --file input.txt --stdout > audio.mp3

# List available voices
rustled --list-voices

# List voices filtered by locale
rustled --list-voices es

# Use a specific voice
rustled --voice en-US-JennyNeural --file input.txt

# Adjust speech rate (-100 to 100)
rustled --rate 50 --file input.txt

# Adjust pitch (-100 to 100)
rustled --pitch -20 --file input.txt

# Adjust volume (-100 to 100)
rustled --volume 30 --file input.txt

# Use SSML input
rustled --ssml --file input.ssml

# Batch mode: process each line separately
rustled --batch --file lines.txt
```

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `-f, --file` | Input file (reads from stdin if not provided) | stdin |
| `-o, --output` | Output file path | `output.mp3` |
| `--stdout` | Write audio to stdout instead of file | false |
| `--voice` | Voice to use (partial match supported) | `es-ES-XimenaNeural` |
| `--list-voices [LOCALE]` | List available voices, optionally filtered by locale | - |
| `--rate` | Speech rate (-100 to 100) | 0 |
| `--pitch` | Speech pitch (-100 to 100) | 0 |
| `--volume` | Speech volume (-100 to 100) | 0 |
| `--ssml` | Input is SSML markup | false |
| `--batch` | Process each line as separate synthesis | false |

## Voice Names

Voices follow the pattern `locale-nameNeural` (e.g., `es-ES-XimenaNeural`, `en-US-JennyNeural`). You can use a partial name for the `--voice` flag (e.g., `Ximena` matches `es-ES-XimenaNeural`).

## Building

```bash
cargo build --release
```

The binary will be in `target/release/rustled`.

## License

MIT
