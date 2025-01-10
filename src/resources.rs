use std::{
    fs::{self, File},
    io::{self, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub mod palette;
pub mod rawdata;
pub mod spriteset;
pub mod tileset;

#[derive(Serialize, Deserialize)]
pub struct ResConfig {
    #[serde(default)]
    palettes: Vec<palette::PaletteDef>,
    #[serde(default)]
    rawdata: Vec<rawdata::RawDataDef>,
    #[serde(default)]
    sprites: Vec<spriteset::SpriteSetDef>,
    #[serde(default)]
    tilesets: Vec<tileset::TileSetDef>,
}

pub fn compile_all(project_path: &Path) -> anyhow::Result<()> {
    let res_path = project_path.join("resources/");

    println!("{}", res_path.join("config.yml").to_str().unwrap());

    let res_config: ResConfig =
        serde_yml::from_reader(File::open(res_path.join("config.yml")).unwrap()).unwrap();

    let _ = fs::create_dir(project_path.join("build"));

    let data_file = fs::File::create(project_path.join("build/resources.bin")).unwrap();
    let mut data_buffer = BufWriter::new(data_file);

    let header_file = fs::File::create(res_path.join("andes_resources.h")).unwrap();
    let mut header_buffer = BufWriter::new(header_file);
    let source_file = fs::File::create(res_path.join("andes_resources.c")).unwrap();
    let mut source_buffer = BufWriter::new(source_file);

    data_buffer.write_all(b"ANDES     ")?;
    data_buffer.seek(SeekFrom::Current(8))?;

    header_buffer
        .write_all(b"// AUTOMATICALLY GENERATED BY ANDES SDK. MODIFYING NOT RECOMMENDED.\n\n")?;
    source_buffer
        .write_all(b"// AUTOMATICALLY GENERATED BY ANDES SDK. MODIFYING NOT RECOMMENDED.\n\n")?;

    header_buffer.write_all(b"#pragma once\n\n#include <andes_res_types.h>\n\n")?;
    source_buffer.write_all(b"#include <andes_resources.h>\n\n")?;

    palette::compile(
        &res_path,
        &res_config,
        &mut data_buffer,
        &mut header_buffer,
        &mut source_buffer,
    )?;
    rawdata::compile(
        &res_path,
        &res_config,
        &mut data_buffer,
        &mut header_buffer,
        &mut source_buffer,
    )?;
    spriteset::compile(
        &res_path,
        &res_config,
        &mut data_buffer,
        &mut header_buffer,
        &mut source_buffer,
    )?;
    tileset::compile(
        &res_path,
        &res_config,
        &mut data_buffer,
        &mut header_buffer,
        &mut source_buffer,
    )?;

    let res_data_length = data_buffer.seek(SeekFrom::Current(0))?.to_le_bytes();
    data_buffer.seek(SeekFrom::Start(10))?;
    data_buffer.write_all(&res_data_length)?;

    data_buffer.seek(SeekFrom::End(0))?;

    Ok(())
}
