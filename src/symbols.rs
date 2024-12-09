use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use goblin::elf::Elf;
use goblin::mach::{self, Mach, MachO};
use goblin::Object;

use crate::errors::*;

#[derive(Clone, Debug, Default)]
pub struct ExtractedSymbol {
    pub name: String,
    pub offset: usize,
    pub size: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct ExtractedSymbols {
    bytes: Option<Vec<u8>>,
    symbols: Vec<ExtractedSymbol>,
}

impl From<Vec<ExtractedSymbol>> for ExtractedSymbols {
    fn from(symbols: Vec<ExtractedSymbol>) -> Self {
        ExtractedSymbols {
            bytes: None,
            symbols,
        }
    }
}

impl ExtractedSymbols {
    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = Some(bytes)
    }

    pub fn dump<P: AsRef<Path>>(&self, output_dir: P) -> Result<(), BRError> {
        let bytes = self
            .bytes
            .as_ref()
            .ok_or(BRError::InternalError("Library code not set"))?;
        fs::create_dir_all(&output_dir)?;
        for symbol in &self.symbols {
            let mut path = PathBuf::new();
            path.push(&output_dir);
            path.push(&symbol.name);
            path.set_extension("bin");
            if let Some(size) = symbol.size {
                println!("{} (offset {}, {} bytes)", symbol.name, symbol.offset, size);
                File::create(path)?.write_all(&bytes[symbol.offset..symbol.offset + size])?;
            }
        }
        Ok(())
    }
}

fn parse_elf(elf: Elf<'_>) -> Result<ExtractedSymbols, BRError> {
    let mut symbols = vec![];

    for symbol in elf
        .dynsyms
        .iter()
        .filter(|symbol| symbol.st_info == 0x12 || symbol.st_info == 0x22)
    {
        let name = elf
            .dynstrtab
            .get_at(symbol.st_name)
            .ok_or(BRError::ParseError)?
            .to_string();
        let extracted_symbol = ExtractedSymbol {
            name,
            offset: symbol.st_value as usize,
            size: match symbol.st_size {
                size if size > 0 => Some(size as usize),
                _ => None,
            },
        };
        symbols.push(extracted_symbol);
    }
    Ok(symbols.into())
}

// Mach-O symbols don't include any sizes, so we need to extract all the symbols
// from the text section, and for each symbol, find the one with the smallest
// offset immediately after the reference symbol, in order to guess the
// reference symbol's size (alignment included).
fn parse_macho(macho: MachO<'_>) -> Result<ExtractedSymbols, BRError> {
    let mut symbols = vec![];

    // Start by finding the boundaries of the text section
    let mut text_offset = None;
    let mut text_size = None;
    for section in macho.segments.sections() {
        for segment in section {
            if let Ok((
                mach::segment::Section {
                    sectname: [b'_', b'_', b't', b'e', b'x', b't', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    segname: [b'_', b'_', b'T', b'E', b'X', b'T', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    size,
                    offset,
                    ..
                },
                _,
            )) = segment
            {
                text_offset = Some(offset as usize);
                text_size = Some(size as usize);
            }
        }
    }
    let text_offset = text_offset.ok_or(BRError::ParseError)?;
    let text_size = text_size.ok_or(BRError::ParseError)?;

    // Extract the symbols we are interested in
    for symbol in macho.symbols.as_ref().ok_or(BRError::ParseError)?.iter() {
        match symbol {
            Ok((
                name,
                mach::symbols::Nlist {
                    n_type: 0xf,
                    n_sect: 1,
                    n_value,
                    ..
                },
            )) if name.len() > 1 && name.starts_with('_') => {
                let extracted_symbol = ExtractedSymbol {
                    name: name[1..].to_string(),
                    offset: n_value as usize,
                    size: None,
                };
                if extracted_symbol.offset < text_offset
                    || extracted_symbol.offset >= text_offset + text_size
                {
                    continue;
                }
                symbols.push(extracted_symbol);
            }
            _ => {}
        }
    }

    // Sort symbols by offset
    symbols.sort_by(|a, b| a.offset.cmp(&b.offset));

    if !symbols.is_empty() {
        let last_offset = symbols.last().unwrap().offset;
        let mut after_last = text_offset + text_size;

        // Find a symbol whose offset is the smallest after the offset
        // of the previously found symbol with the highest offset
        for symbol in macho.symbols.as_ref().ok_or(BRError::ParseError)?.iter() {
            if let Ok((
                _name,
                mach::symbols::Nlist {
                    n_sect: 1, n_value, ..
                },
            )) = symbol
            {
                let offset = n_value as usize;
                if offset <= last_offset || offset >= after_last {
                    continue;
                }
                after_last = offset;
            }
        }

        // Compute sizes
        let last_i = symbols.len() - 1;
        for i in 0..last_i {
            symbols[i].size = Some(symbols[i + 1].offset - symbols[i].offset)
        }
        symbols[last_i].size = Some(after_last - last_offset);
    }
    Ok(symbols.into())
}

pub fn exported_symbols<P: AsRef<Path>>(path: P) -> Result<ExtractedSymbols, BRError> {
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;
    let mut symbols = match Object::parse(&buffer).map_err(|_| BRError::ParseError)? {
        Object::Mach(Mach::Binary(macho)) => parse_macho(macho),
        Object::Elf(elf) => parse_elf(elf),
        _ => { return Err(BRError::Unsupported) },
    }?;
    symbols.set_bytes(buffer);
    Ok(symbols)
}
