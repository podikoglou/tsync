use crate::header::Header;
use anyhow::bail;
use byteorder::ByteOrder;
use postcard::{from_bytes, to_allocvec};
use std::io::{BufRead, Write};

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

macro_rules! write_struct {
    ($writer: expr, $struct: expr) => {
        let serialized = to_allocvec(&$struct)?;

        // todo: maybe we can do it using the byteorder crate just inc ase
        $writer.write(&serialized.len().to_le_bytes())?;
        $writer.write(&serialized)?;
    };
}

macro_rules! read_struct {
    ($reader: expr, $buf: expr, $type: ty) => {{
        // resize buffer to 8 bytes and read u32 (size)
        $buf.resize(8, 0x00);
        $reader.read_exact(&mut $buf)?;

        let size = byteorder::LittleEndian::read_u32(&$buf);

        // resize buffer to size, read struct data
        $buf.resize(size as usize, 0x00);
        $reader.read_exact(&mut $buf)?;

        from_bytes::<$type>(&mut $buf)?
    }};
}

pub fn write_file_metadata<W: Write>(writer: &mut W, file: &FileMetadata) -> anyhow::Result<()> {
    let name_header = Header::Name(file.name.clone());
    let pieces_header = Header::Pieces(file.pieces_amount);

    write_struct!(writer, name_header);
    write_struct!(writer, pieces_header);

    Ok(())
}

pub fn read_file_metadata<R: BufRead>(reader: &mut R) -> anyhow::Result<FileMetadata> {
    let mut buf: Vec<u8> = vec![0; 8];

    // read names header
    let name_header = read_struct!(reader, buf, crate::proto::Header);

    let name = match name_header {
        Header::Name(val) => val,
        other => bail!("header must be name header, not {:?}", other),
    };

    // read pieces amount header
    let pieces_amount_header = read_struct!(reader, buf, crate::proto::Header);

    let pieces_amount = match pieces_amount_header {
        Header::Pieces(val) => val,
        other => bail!("header must be pieces header, not {:?}", other),
    };

    Ok(FileMetadata {
        name,
        pieces_amount,
    })
}

pub fn write_piece<W: Write>(writer: &mut W, piece: &Piece) -> anyhow::Result<()> {
    let id_header = Header::Id(piece.id);
    let size_header = Header::Size(piece.size);

    write_struct!(writer, id_header);
    write_struct!(writer, size_header);

    writer.write(&piece.data)?;
    writer.flush()?;

    Ok(())
}

pub fn read_piece<R: BufRead>(reader: &mut R) -> anyhow::Result<Piece> {
    let mut buf: Vec<u8> = vec![0; 8];

    // read id header
    let id_header = read_struct!(reader, buf, crate::proto::Header);

    let id = match id_header {
        Header::Id(val) => val,
        other => bail!("header must be id header, not {:?}", other),
    };

    // read size header
    let size_header = read_struct!(reader, buf, crate::proto::Header);

    let size = match size_header {
        Header::Size(val) => val,
        other => bail!("header must be size header, not {:?}", other),
    };

    // read data
    let mut data_buf = vec![0; size];

    reader.read_exact(&mut data_buf)?;

    Ok(Piece {
        id,
        size,
        data: data_buf,
    })
}
