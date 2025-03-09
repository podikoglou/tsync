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
    #[argh(option, default = "8080")]
    pub port: u16,
}

/// Copy a file to a remote server
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "cp")]
pub struct CpCommand {
    /// source file path
    #[argh(positional)]
    pub source: PathBuf,

    /// destination in format "ip:path"
    #[argh(positional)]
    pub destination: Destination,
}

#[derive(Debug)]
pub struct Destination {
    pub ip: IpAddr,
    pub path: PathBuf,
}

impl FromStr for Destination {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err("Destination must be in format 'ip:path'".to_string());
        }

        let ip = parts[0]
            .parse::<IpAddr>()
            .map_err(|e| format!("Invalid IP address: {}", e))?;
        let path = PathBuf::from(parts[1]);

        Ok(Destination { ip, path })
    }
}
