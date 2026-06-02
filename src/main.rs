use clap::Parser;
use msedge_tts::voice::get_voices_list;
use msedge_tts::tts::client::connect;
use msedge_tts::tts::SpeechConfig;
use std::io::{self, Read, Write};
use tracing::debug;
use tracing_subscriber::fmt;

#[derive(Parser)]
#[command(name = "rustled")]
#[command(about = "Converts text to MP3 audio using Microsoft Edge TTS", long_about = None)]
struct Cli {
    /// Input file to convert (if not provided, reads from stdin)
    #[arg(short, long)]
    file: Option<String>,

    /// List available voices, optionally filter by locale (e.g. es, en-US)
    #[arg(long, short)]
    list_voices: Option<Option<String>>,

    /// Voice to use for synthesis (e.g. es-ES-XimenaNeural, Ximena). Partial match supported.
    #[arg(long)]
    voice: Option<String>,

    /// Output file (default: output.mp3). Ignored when --stdout is used.
    #[arg(short, long, default_value = "output.mp3")]
    output: String,

    /// Write audio to stdout instead of file
    #[arg(long)]
    stdout: bool,

    /// Speech rate (-100 to 100, default: 0)
    #[arg(long, default_value_t = 0)]
    rate: i32,

    /// Speech pitch (-100 to 100, default: 0)
    #[arg(long, default_value_t = 0)]
    pitch: i32,

    /// Speech volume (-100 to 100, default: 0)
    #[arg(long, default_value_t = 0)]
    volume: i32,

    /// Input is SSML markup instead of plain text
    #[arg(long)]
    ssml: bool,

    /// Process each line of input as a separate synthesis with every matching voice.
    /// Combine with --voice for a subset, or omit for all voices.
    #[arg(long)]
    batch: bool,
}

fn main() {
    #[cfg(debug_assertions)]
    fmt().with_max_level(tracing::Level::DEBUG).with_writer(std::io::stderr).init();
    #[cfg(not(debug_assertions))]
    fmt().with_max_level(tracing::Level::INFO).with_writer(std::io::stderr).init();

    let cli = Cli::parse();

    let voices = get_voices_list().expect("Failed to list voices");

    if let Some(filter) = &cli.list_voices {
        let filtered: Vec<_> = match filter {
            Some(locale) => voices.iter().filter(|v| {
                v.locale.as_deref().unwrap_or("").to_lowercase().contains(&locale.to_lowercase())
            }).collect(),
            None => voices.iter().collect(),
        };
        for voice in filtered {
            println!("{} - {}", voice.name, voice.locale.as_deref().unwrap_or(""));
        }
        return;
    }

    let selected_voice = cli.voice.as_deref().unwrap_or("XimenaNeural").to_lowercase();
    let raw_input = if let Some(path) = &cli.file {
        debug!("Reading input from file: {path}");
        std::fs::read_to_string(path).expect("Failed to read file")
    } else {
        debug!("Reading input from stdin");
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read stdin");
        buffer
    };

    debug!("Voice: {selected_voice}");
    debug!("Rate: {}", cli.rate);
    debug!("Pitch: {}", cli.pitch);
    debug!("Volume: {}", cli.volume);
    debug!("SSML: {}", cli.ssml);
    debug!("Batch: {}", cli.batch);
    debug!("Stdout: {}", cli.stdout);
    debug!("Input length: {} characters", raw_input.len());

    let matched_voices: Vec<_> = voices.iter().filter(|v| {
        v.name.to_lowercase().contains(&selected_voice)
    }).collect();

    if matched_voices.is_empty() {
        eprintln!("No voice found matching '{selected_voice}'. Use --list-voices to see available voices.");
        std::process::exit(1);
    }

    let lines: Vec<&str> = if cli.batch {
        raw_input.lines().collect()
    } else {
        vec![raw_input.as_str()]
    };

    for voice in &matched_voices {
        let mut config = SpeechConfig::from(*voice);
        config.rate = cli.rate;
        config.pitch = cli.pitch;
        config.volume = cli.volume;

        let mut tts = connect().expect("Failed to connect to TTS service");

        for (i, line) in lines.iter().enumerate() {
            let text = line.trim();
            if text.is_empty() {
                continue;
            }

            let input = if cli.ssml { text.to_string() } else { text.to_string() };

            debug!("Synthesizing: voice={}, chunk={}, len={}", voice.name, i, input.len());
            let audio_data = tts
                .synthesize(&input, &config)
                .expect("Failed to generate audio");

            if cli.stdout {
                let stdout = io::stdout();
                let mut handle = stdout.lock();
                handle.write_all(&audio_data.audio_bytes).expect("Failed to write to stdout");
            } else {
                let filename = if cli.batch {
                    let safe_name = voice.name.replace(' ', "_");
                    format!("{}-{}.mp3", safe_name, i)
                } else {
                    cli.output.clone()
                };
                std::fs::write(&filename, &audio_data.audio_bytes)
                    .expect("Failed to write audio to file");
                if cli.batch {
                    println!("Wrote {filename}");
                }
            }
        }
    }
}
