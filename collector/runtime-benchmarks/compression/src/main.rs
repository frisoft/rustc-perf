use std::io::Write;

use brotli::enc::BrotliEncoderParams;

use benchlib::benchmark::run_benchmark_group;

const TEXT_SHERLOCK: &str = include_str!("../../data/sherlock.txt");

fn compress(data: &str) -> Vec<u8> {
    let mut target: Vec<u8> = Vec::with_capacity(1024 * 1024);

    let mut writer =
        brotli::CompressorWriter::with_params(&mut target, 4096, &BrotliEncoderParams::default());
    std::io::copy(&mut data.as_bytes(), &mut writer).unwrap();
    writer.flush().unwrap();
    drop(writer);
    target
}

fn main() {
    // Decompression is much faster than compression, so inflate the data a bit
    let compressed_text = compress(&TEXT_SHERLOCK.repeat(20));

    run_benchmark_group(|group| {
        group.register_benchmark("brotli-compress", || {
            let mut target_buffer: Vec<u8> = Vec::with_capacity(1024 * 1024);
            let mut params = BrotliEncoderParams::default();
            params.quality = 10;

            move || {
                let mut writer =
                    brotli::CompressorWriter::with_params(&mut target_buffer, 4096, &params);
                std::io::copy(&mut TEXT_SHERLOCK.as_bytes(), &mut writer).unwrap();
                writer.flush().unwrap();
                drop(writer);
                target_buffer
            }
        });
        group.register_benchmark("brotli-decompress", || {
            let mut buffer: Vec<u8> = Vec::with_capacity(TEXT_SHERLOCK.len() * 2);
            || {
                let mut reader = brotli::Decompressor::new(compressed_text.as_slice(), 4096);
                std::io::copy(&mut reader, &mut buffer).unwrap();
                buffer
            }
        });
    });
}
