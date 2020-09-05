#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::exit;

mod window;

const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

#[derive(Debug, Clone)]
struct IHDR {
    image_width: u32,
    image_height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

#[derive(Debug, Clone)]
struct BKGD {
    r: u16,
    g: u16,
    b: u16,
}

fn main() -> io::Result<()> {
    println!("");
    let mut file = File::open("270.png")?;

    let mut header = [0; 8];
    file.read(&mut header)?;
    if header == PNG_HEADER {
        println!("png detected\n")
    } else {
        println!("file is not a png\n");
        exit(1);
    }

    let ihdr_chunk_header = read_chunk_header(&file)?;
    if ihdr_chunk_header.chunk_type != "IHDR" {
        println!("ihdr chunk not found. aborting...");
        exit(1);
    }
    let ihdr = read_ihdr_data(&file)?;

    let mut image_data = vec![[0u8; 4]];

    let mut bkgd = None;
    let mut done = false;

    while !done {
        let chunk_header = read_chunk_header(&file)?;

        match chunk_header.chunk_type.as_str() {
            "IHDR" => {
                println!("multiple ihdr headers. aborting....");
                exit(1);
            }
            "IEND" => println!("finished reading file\n"),
            "bKGD" => {
                bkgd = Some(read_bkgd_data(&file, &chunk_header, &ihdr)?);
            }
            "IDAT" => {
                let px = chunk_header.size / 4;
                let lines = px / ihdr.image_width;
                println!("chunk contains {} pixels over {} lines", px, lines);
                if lines > 0 {
                    for _ in 0..lines {
                        for _ in 0..ihdr.image_width {
                            let mut buf = [0u8; 4];
                            file.read(&mut buf)?;
                            image_data.push(buf.to_owned());
                        }
                    }
                } else {
                    // Ignoring idat chunks which don't fill a line
                    let mut _buf = vec![0u8; chunk_header.size as usize];
                    file.read(&mut _buf)?;
                }
            }
            _ => {
                let mut _buffer = vec![0u8; chunk_header.size as usize];
                file.read(&mut _buffer)?;
                println!("{} chunk ignored...\n", chunk_header.chunk_type);
            }
        }

        discard_crc(&file)?;

        if chunk_header.size == 0 {
            done = true;
        }
    }

    window::run(
        ihdr.image_width,
        ihdr.image_height,
        move |frame: &mut [u8]| {
            let image_data = image_data.clone();
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let rgba = image_data.get(i).unwrap_or(&[0, 0, 0, 0]);
                pixel.copy_from_slice(&*rgba);
            }
        },
    );

    println!("data... ");

    println!("{:?}", ihdr);

    if let Some(bkgd) = bkgd {
        println!("{:?}", bkgd);
    }

    Ok(())
}

fn discard_crc(mut file: &File) -> Result<(), io::Error> {
    // I don't care about the crc
    let mut _crc = [0; 4];
    file.read(&mut _crc)?;
    Ok(())
}

#[derive(Debug)]
struct ChunkHeader {
    size: u32,
    chunk_type: String,
}

fn read_chunk_header(mut file: &File) -> Result<ChunkHeader, io::Error> {
    let mut chunk_size = [0u8; 4];
    let mut chunk_type = [0u8; 4];
    file.read(&mut chunk_size)?;
    file.read(&mut chunk_type)?;
    return Ok(ChunkHeader {
        size: u32::from_be_bytes(chunk_size),
        chunk_type: String::from_utf8(chunk_type.to_vec()).unwrap(),
    });
}

fn read_ihdr_data(mut file: &File) -> Result<IHDR, io::Error> {
    let mut width = [0u8; 4];
    let mut height = [0u8; 4];
    let mut bit_depth = [0u8; 1];
    let mut color_type = [0u8; 1];
    let mut compression_method = [0u8; 1];
    let mut filter_method = [0u8; 1];
    let mut interlace_method = [0u8; 1];

    file.read(&mut width)?;
    file.read(&mut height)?;
    file.read(&mut bit_depth)?;
    file.read(&mut color_type)?;
    file.read(&mut compression_method)?;
    file.read(&mut filter_method)?;
    file.read(&mut interlace_method)?;

    discard_crc(&file)?;

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

fn read_bkgd_data(mut file: &File, header: &ChunkHeader, ihdr: &IHDR) -> Result<BKGD, io::Error> {
    if ihdr.color_type == 6 {
        let mut r = [0u8; 2];
        let mut g = [0u8; 2];
        let mut b = [0u8; 2];
        file.read(&mut r)?;
        file.read(&mut g)?;
        file.read(&mut b)?;
        return Ok(BKGD {
            r: u16::from_be_bytes(r),
            g: u16::from_be_bytes(g),
            b: u16::from_be_bytes(b),
        });
    } else {
        println!(
            "color_type {} is not supported. ignoring bKGD",
            ihdr.color_type
        );
        let mut _buf = vec![0u8; header.size as usize];
        file.read(&mut _buf)?;
    }

    Ok(BKGD { r: 0, g: 0, b: 0 })
}
