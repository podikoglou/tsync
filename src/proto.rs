use std::{
    io::{BufRead, Write},
    str::FromStr,
};

use anyhow::bail;

use crate::header::Header;

#[derive(Debug)]
pub struct FileMetadata {
    pub name: String,
    pub pieces_amount: usize,
}

#[derive(Debug)]
pub struct Piece {
    pub id: usize,
    pub size: usize,
    pub data: Vec<u8>,
    // TODO: checksum!
}

pub fn write_file_metadata<W: Write>(writer: &mut W, file: &FileMetadata) -> anyhow::Result<()> {
    writer.write(Header::Name(file.name.clone()).to_string().as_bytes())?;
    writer.write(Header::Pieces(file.pieces_amount).to_string().as_bytes())?;

    dbg!(Header::Pieces(file.pieces_amount).to_string());

    Ok(())
}

pub fn read_file_metadata<R: BufRead>(reader: &mut R) -> anyhow::Result<FileMetadata> {
    let mut buf = String::default();

    // read name header
    reader.read_line(&mut buf)?;

    let name = match Header::from_str(&buf)? {
        Header::Name(val) => val,
        other => bail!("header must be name header, not {:?}", other),
    };

    // read pieces header
    reader.read_line(&mut buf)?;

    let pieces_amount = match Header::from_str(&buf)? {
        Header::Pieces(val) => val,
        other => bail!("header must be pieces header, not {:?}", other),
    };

    Ok(FileMetadata {
        name,
        pieces_amount,
    })
}

pub fn write_piece<W: Write>(writer: &mut W, piece: &Piece) -> anyhow::Result<()> {
    writer.write(Header::Id(piece.id).to_string().as_bytes())?;
    writer.write(Header::Size(piece.size).to_string().as_bytes())?;

    writer.write(&piece.data)?;

    Ok(())
}

pub fn read_piece<R: BufRead>(reader: &mut R) -> anyhow::Result<Piece> {
    let mut header_buf = String::default();

    // read id header
    reader.read_line(&mut header_buf)?;

    let id = match Header::from_str(&header_buf)? {
        Header::Id(val) => val,
        other => bail!("header must be id header, not {:?}", other),
    };

    // read size header
    reader.read_line(&mut header_buf)?;

    let size = match Header::from_str(&header_buf)? {
        Header::Size(val) => val,
        other => bail!("header must be size header, not {:?}", other),
    };

    // read data
    let mut data_buf = Vec::with_capacity(size);

    reader.read_exact(&mut data_buf)?;

    Ok(Piece {
        id,
        size,
        data: data_buf,
    })
}
