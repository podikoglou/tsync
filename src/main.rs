use args::{Command, CpCommand, RsyncClone, ServerCommand};
use proto::{
    read_file_metadata, read_piece, write_file_metadata, write_piece, FileMetadata, Piece,
};
use std::cmp::min;
use std::io::{Read, Seek};
use std::net::SocketAddr;
use std::{
    fs::File,
    io::BufReader,
    net::{TcpListener, TcpStream},
    os::unix::fs::MetadataExt,
};

pub mod args;
pub mod header;
pub mod proto;

const PIECE_SIZE: u64 = 1024 * 1024; // TODO: convert to usize
const PORT: u16 = 8686;

pub fn calc_offsets(file_size: u64, piece_size: u64) -> Vec<u64> {
    let mut curr_offset: u64 = 0;

    let mut chunks: Vec<u64> = vec![];

    while curr_offset < file_size {
        chunks.push(curr_offset);

        curr_offset = min(curr_offset + piece_size, file_size);
    }

    chunks
}

pub fn handle_client(stream: TcpStream) -> anyhow::Result<()> {
    let mut reader = BufReader::new(stream);

    loop {
        let metadata = read_file_metadata(&mut reader)?;

        for _ in 0..metadata.pieces_amount {
            let piece = read_piece(&mut reader)?;
            dbg!(piece.id, piece.data.len());
        }
    }
}

fn copy_file(cmd: CpCommand) -> anyhow::Result<()> {
    println!(
        "Copying file from {} to {}:{}",
        cmd.source.display(),
        cmd.destination.ip,
        cmd.destination.path.display()
    );

    // open connection
    let mut stream = TcpStream::connect(SocketAddr::new(cmd.destination.ip, PORT))?;

    // split into pieces
    let metadata = std::fs::metadata(&cmd.source)?;
    let pieces_offsets = calc_offsets(metadata.size(), PIECE_SIZE);

    // write metadata
    let file_name = cmd.source.file_name().unwrap().to_str().unwrap();

    let metadata = FileMetadata {
        name: file_name.to_string(),
        pieces_amount: pieces_offsets.len(),
    };

    write_file_metadata(&mut stream, &metadata)?;

    // open file
    let file = File::open(&cmd.source)?;
    let mut reader = BufReader::new(file);

    let mut buf = [0; PIECE_SIZE as usize];

    // read each piece and send it
    for (idx, offset) in pieces_offsets.iter().enumerate() {
        reader.seek(std::io::SeekFrom::Start(*offset))?;
        reader.read(&mut buf)?;

        let piece = Piece {
            id: idx,
            size: buf.len(),
            data: buf.to_vec(),
        };

        write_piece(&mut stream, &piece)?;
    }

    Ok(())
}

fn run_server(_cmd: ServerCommand) -> anyhow::Result<()> {
    // use constant
    let listener = TcpListener::bind("127.0.0.1:8686")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: RsyncClone = argh::from_env();

    match args.command {
        Command::Cp(cmd) => copy_file(cmd),
        Command::Server(cmd) => run_server(cmd),
    }
}
