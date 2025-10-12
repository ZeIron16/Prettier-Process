### README.md

# Chuka

Chuka is a shell-like multitool application that includes different functionalities, such as an audio/video downloader (from a link) and a file compressor/decompressor.

## Features

* **Integrated GUI Shell**: The application provides a simple, shell-like GUI built with `eframe` and `egui`.
* **Media Downloader**: Download videos or audio from a given URL using the `yt-dlp` utility.
* **File Compression**: Compress files using various algorithms to try to optimize it, with support for:
    * `.zst` (Zstandard) and `.xz` (LZMA2) for text and data files (`.txt`, `.csv`, `.json`, `.py`, `.c`, `.rs`, `.pdf`, `.img`).
    * `.br` (Brotli) for web-related files (`.html`, `.js`, `.css`, `.svg`).
    * `.7z` (7-Zip) for general files and folders.
    * `libx265` for efficient video compression (`.mp4`, `.mkv`, `.mov`) using `ffmpeg`.
    * `.flac` for WAV audio files (`.wav`).
* **File Decompression**: Decompress files with `.zst`, `.xz`, `.br`, and `.7z` extensions.

## Usage

Here is a list of the available commands:

| Command | Description | Options |
|---------|-------------|---------|
| `clear` | Clear the terminal output. | |
| `help` | Display the help menu with a list of all commands and their arguments. | |
| `exit` | Leaves the application. | |
| `dwn <url>` | Downloads a video or audio from a specified URL. | `-a`, `--audio`: Downloads only the audio in MP3 format.<br>`-o`, `--output <file>`: Specifies the name of the output file. |
| `compress <input> <output>` | Compresses a file or folder with the optimal extension. | `-m`, `--max`: Activates maximal compression, which may take longer. |
| `decompress <input>` | Decompresses a file or folder. | |

## Dependencies

The project is built with Rust and uses the following main dependencies:

* `eframe` and `egui` : For the GUI.
* `clap` : For handling command-line arguments.
* `sevenz-rust`, `xz2`, `zstd`, `brotli2`, `brotli` : For various compression and decompression tasks.
* `hound` and `flacenc` : For audio compression.

## Installation

To build and run the project, you need to have Rust installed. You can then build the project from source:

```bash
cargo build --release
```

After building, you can run the application with:

```bash
cargo run --release
```
