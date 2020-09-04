#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[derive(Debug)]
struct IHDR {
    image_width: u32,
    image_height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

fn main() -> io::Result<()> {
    println!("");
    let mut file = File::open("270.png")?;

    let mut header = [0; 8];
    file.read(&mut header[..])?;
    if header == PNG_HEADER {
        println!("png detected\n")
    } else {
        println!("file is not a png\n");
        exit(1);
    }

    let mut ihdr = None;
    let mut bkgd = None;

    let mut done = false;
    while !done {
        let chunk_header = read_chunk_header(&file)?;

        match chunk_header.chunk_type.as_str() {
            "IHDR" => {
                ihdr = Some(read_ihdr_data(&file)?);
            }
            "IEND" => println!("finished reading file\n"),
            "bKGD" => {
                bkgd = Some(read_bkgd_data(&file, &chunk_header, &ihdr)?);
            }
            _ => {
                let mut _buffer = vec![0u8; chunk_header.size as usize];
                file.read(&mut _buffer[..])?;
                println!("{} chunk ignored...\n", chunk_header.chunk_type);
            }
        }

        // I don't care about the crc
        let mut _crc = [0; 4];
        file.read(&mut _crc[..])?;

        if chunk_header.size == 0 {
            done = true;
        }
    }

    if let Some(ihdr) = ihdr {
        println!("{:?}", ihdr)
    }

    Ok(())
}

#[derive(Debug)]
struct ChunkHeader {
    size: u32,
    chunk_type: String,
}

fn read_chunk_header(mut file: &File) -> Result<ChunkHeader, std::io::Error> {
    let mut chunk_size = [0u8; 4];
    let mut chunk_type = [0u8; 4];
    file.read(&mut chunk_size[..])?;
    file.read(&mut chunk_type[..])?;
    return Ok(ChunkHeader {
        size: u32::from_be_bytes(chunk_size),
        chunk_type: String::from_utf8(chunk_type.to_vec()).unwrap(),
    });
}

fn read_ihdr_data(mut file: &File) -> Result<IHDR, std::io::Error> {
    let mut width = [0u8; 4];
    let mut height = [0u8; 4];
    let mut bit_depth = [0u8; 1];
    let mut color_type = [0u8; 1];
    let mut compression_method = [0u8; 1];
    let mut filter_method = [0u8; 1];
    let mut interlace_method = [0u8; 1];

    file.read(&mut width[..])?;
    file.read(&mut height[..])?;
    file.read(&mut bit_depth[..])?;
    file.read(&mut color_type[..])?;
    file.read(&mut compression_method[..])?;
    file.read(&mut filter_method[..])?;
    file.read(&mut interlace_method[..])?;

    return Ok(IHDR {
        image_width: u32::from_be_bytes(width),
        image_height: u32::from_be_bytes(height),
        bit_depth: bit_depth[0],
        color_type: color_type[0],
        compression_method: compression_method[0],
        filter_method: filter_method[0],
        interlace_method: interlace_method[0],
    });
}

struct BKGD {}

fn read_bkgd_data(
    mut file: &File,
    header: &ChunkHeader,
    ihdr: &Option<IHDR>,
) -> Result<BKGD, std::io::Error> {
    let mut _buf = vec![0u8; header.size as usize];
    file.read(&mut _buf)?;

    Ok(BKGD {})
}
