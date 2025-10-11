use std::fs::File;
use std::io::copy;
use std::io::{self, BufReader, BufWriter};
use xz2::write::XzEncoder;
use xz2::read::XzDecoder;
use zstd::stream::Encoder;
use zstd::stream::copy_decode;
use sevenz_rust;
use brotli::CompressorWriter;
use brotli2::read::BrotliDecoder;
use std::process::Command;
use hound;
use flacenc::component::BitRepr;
use flacenc::error::Verify;
use flacenc::bitsink::ByteSink;

pub fn compress(in_path: &str, out_path: &str, max: bool) -> Result<(), Box<dyn std::error::Error>>{
    let input = File::open(in_path)?;
    let mut reader = BufReader::new(input);
    let ext = if get_ext(in_path) != None {get_ext(in_path).unwrap()} else{""};
    
    match in_path{
        p if is_to_zst(p) => {
            if max{
                let output = File::create(format!("{out_path}.{ext}.xz"))?;
                let mut writer = XzEncoder::new(BufWriter::new(output), 9);
                io::copy(&mut reader, &mut writer)?;
                writer.finish()?;
            }
            else{
                let output = File::create(format!("{out_path}.{ext}.zst"))?;
                let mut writer = Encoder::new(BufWriter::new(output), 10)?;
                io::copy(&mut reader, &mut writer)?;
                writer.finish()?;
            }
        }
        p if p.ends_with(".html") || p.ends_with(".js") || p.ends_with(".css") || p.ends_with(".svg") => {
            let level = if max {11} else{5};
            let output = File::create(format!("{out_path}.{ext}.br"))?;
            let writer = BufWriter::new(output);
            let mut compressor = CompressorWriter::new(writer, 4096, level, 22);
            copy(&mut reader, &mut compressor)?;
        }
        p if p.ends_with(".mp4") || p.ends_with(".mkv") || p.ends_with(".mov")=> {
            let ext = &p[p.len() - 4..];
            if max{
                Command::new("ffmpeg")
                .args(["-i", in_path, "-c:v", "libx265", "-crf", "28", &format!("{out_path}{ext}")])
                .status()?;
            }
            else{
                Command::new("ffmpeg")
                .args(["-i", in_path, "-c:v", "libx265", "-crf", "23", "-c:a", "aac", "-b:a", "128k", &format!("{out_path}{ext}")])
                .status()?;
            }
        }
        p if p.ends_with(".wav") => {
            comp_audio(in_path, out_path)?
        }
        _ => {sevenz_rust::compress_to_path(in_path, format!("{out_path}.{ext}.7z"))?;}
    }
    
    Ok(())
}


fn is_to_zst(p: &str) -> bool{
    if p.ends_with(".txt") || 
        p.ends_with(".csv") || 
        p.ends_with(".json") || 
        p.ends_with(".log") || 
        p.ends_with(".py") || 
        p.ends_with(".c") || 
        p.ends_with(".rs") || 
        p.ends_with(".pdf") ||
        p.ends_with(".img") ||
        p.ends_with(".img") {
            return true;
        }
    false
}

fn get_ext(path: &str) -> Option<&str> {
    path.rfind('.').map(|pos| &path[pos + 1..])
}

fn comp_audio(in_path: &str, out_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(in_path)?;
    let spec = reader.spec();
    
    let samples: Vec<i32> = match spec.bits_per_sample {
        16 => reader.samples::<i16>()
            .map(|s| s.map(|v| v as i32))
            .collect::<Result<Vec<_>, _>>()?,
        24 | 32 => reader.samples::<i32>()
            .collect::<Result<Vec<_>, _>>()?,
        _ => return Err("Unsupported bit depth".into()),
    };
    
    let config = flacenc::config::Encoder::default()
        .into_verified()
        .map_err(|e| format!("Config error: {:?}", e))?;
    
    let source = flacenc::source::MemSource::from_samples(
        &samples,
        spec.channels as usize,
        spec.bits_per_sample as usize,
        spec.sample_rate as usize,
    );
    
    let flac_stream = flacenc::encode_with_fixed_block_size(
        &config,
        source,
        config.block_size,
    )?;
    
    let mut sink = ByteSink::new();
    flac_stream.write(&mut sink)?;
    
    std::fs::write(format!("{out_path}.flac"), sink.as_slice())?;
    
    Ok(())
}

pub fn decompress(in_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    match in_path{
        p if p.ends_with(".zst") => {
            copy_decode(File::open(in_path)?, File::create(in_path.trim_end_matches(".zst"))?)?;
        }
        p if p.ends_with(".xz") => {
            let mut d = XzDecoder::new(File::open(in_path)?);
            copy(&mut d, &mut File::create(in_path.trim_end_matches(".xz"))?)?;
        }
        p if p.ends_with(".br") => {
            let mut d = BrotliDecoder::new(File::open(in_path)?);
            copy(&mut d, &mut File::create(in_path.trim_end_matches(".br"))?)?;
        }
        p if p.ends_with(".7z") => {
            sevenz_rust::decompress_file(in_path, in_path.trim_end_matches(".7z"))?;
        }
        _ => println!("Unsupported extension")
    }

    Ok(())
}