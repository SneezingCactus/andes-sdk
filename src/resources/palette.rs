use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use super::ResConfig;

#[derive(Serialize, Deserialize)]
pub struct PaletteDef {
    id: String,
    path: String,
}

pub fn compile(
    res_path: &PathBuf,
    res_config: &ResConfig,
    _data_buffer: &mut BufWriter<File>,
    header_buffer: &mut BufWriter<File>,
    source_buffer: &mut BufWriter<File>,
) -> anyhow::Result<()> {
    header_buffer.write_all(b"\n// ---- palettes ----\n")?;
    source_buffer.write_all(b"\n// ---- palettes ----\n")?;

    for item in res_config.palettes.iter() {
        let content = fs::read_to_string(res_path.join(&item.path))?;
        let content_lines = content.split('\n');

        let mut color_amount = 0;

        source_buffer.write_fmt(format_args!(
            "const unsigned short DATA_{}[] = {{ ",
            item.id
        ))?;

        for line in content_lines.skip(3) {
            if line.is_empty() {
                continue;
            }

            let channels: Vec<u16> = line
                .split(' ')
                .map(|x| {
                    x.parse::<u16>()
                        .with_context(|| format!("Failed to parse palette \"{}\"", item.id))
                })
                .collect::<anyhow::Result<Vec<u16>>>()?;

            let color: u16 = channels[0] >> 4 | channels[1] >> 4 << 4 | channels[2] >> 4 << 8;

            source_buffer.write_fmt(format_args!("0x{:x}, ", color))?;
            color_amount += 1;
        }

        source_buffer.write_fmt(format_args!(" }};\n",))?;

        header_buffer.write_fmt(format_args!(
            "extern struct PaletteResource RES_{};\n",
            item.id
        ))?;
        source_buffer.write_fmt(format_args!(
            "struct PaletteResource RES_{} = {{ .data = &DATA_{}, .size = {} }};\n",
            item.id,
            item.id,
            color_amount * 2
        ))?;
    }

    Ok(())
}
