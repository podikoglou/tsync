use std::{net::IpAddr, path::PathBuf, str::FromStr};

use argh::FromArgs;

/// A simple rsync-like program
#[derive(FromArgs, Debug)]
pub struct RsyncClone {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    Server(ServerCommand),
    Cp(CpCommand),
}

/// Start the file transfer server
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "server")]
pub struct ServerCommand {
    /// port to listen on
    #[argh(option, default = "8086")]
    pub port: u16,
}

/// Copy a file to a remote server
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "cp")]
pub struct CpCommand {
    /// source file path
    #[argh(positional)]
    pub source: PathBuf,

    /// destination in format "ip:[port:]path". (default port: 8086)
    #[argh(positional)]
    pub destination: Destination,
}

#[derive(Debug)]
pub struct Destination {
    pub ip: IpAddr,
    pub port: u16,
    pub path: PathBuf,
}

impl FromStr for Destination {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        let ip = parts[0]
            .parse::<IpAddr>()
            .map_err(|e| format!("Invalid IP address: {}", e))?;

        let port = parts
            .get(1)
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8086);

        let path = PathBuf::from(parts.last().unwrap_or(&""));

        Ok(Destination { ip, port, path })
    }
}
