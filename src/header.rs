use anyhow::{bail, ensure};
use std::{
    fmt::{write, Display},
    str::FromStr,
};

const PREFIX: &str = "##TSYNC_";
const SUFFIX: &str = "##";

#[derive(Debug, PartialEq)]
pub enum Header {
    Id(usize),
    Size(usize),
    Pieces(usize),
    Name(String),
}

impl FromStr for Header {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();

        ensure!(input.starts_with(PREFIX), "must start with prefix");
        ensure!(input.ends_with(SUFFIX), "must start with prefix");
        ensure!(input.contains("="), "must contain a =");

        let pair = &input[PREFIX.len()..input.len() - SUFFIX.len()];

        let mut split = pair.splitn(2, "=").collect::<Vec<&str>>();

        // NOTE: I don't know why this works and I'm terrified to find out
        let value = split.pop().unwrap();
        let key = split.pop().unwrap();

        match key {
            "ID" => Ok(Header::Id(value.parse()?)),
            "SIZE" => Ok(Header::Size(value.parse()?)),
            "PIECES" => Ok(Header::Pieces(value.parse()?)),
            "NAME" => Ok(Header::Name(value.to_string())),
            other => bail!("invalid header {}", other),
        }
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        match self {
            Header::Id(value) => format!("##TSYNC_ID={}##\n", value),
            Header::Size(value) => format!("##TSYNC_SIZE={}##\n", value),
            Header::Pieces(value) => format!("##TSYNC_PIECES={}##\n", value),
            Header::Name(value) => format!("##TSYNC_NAME={}##\n", value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::header::Header;
    use std::str::FromStr;

    #[test]
    fn test_id_header_from_str() -> anyhow::Result<()> {
        assert_eq!(Header::from_str("##TSYNC_ID=488##")?, Header::Id(488));

        Ok(())
    }

    #[test]
    fn test_size_header_from_str() -> anyhow::Result<()> {
        assert_eq!(
            Header::from_str("##TSYNC_SIZE=49238##")?,
            Header::Size(49238)
        );

        Ok(())
    }

    #[test]
    fn test_pieces_header_from_str() -> anyhow::Result<()> {
        assert_eq!(
            Header::from_str("##TSYNC_PIECES=49238##")?,
            Header::Pieces(49238)
        );

        Ok(())
    }

    #[test]
    fn test_name_header_from_str() -> anyhow::Result<()> {
        assert_eq!(
            Header::from_str("##TSYNC_NAME=aaa.zip##")?,
            Header::Name("aaa.zip".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_header_id_to_string() {
        let header = Header::Id(42);
        assert_eq!(header.to_string(), "##TSYNC_ID=42##\n");
    }

    #[test]
    fn test_header_size_to_string() {
        let header = Header::Size(1024);
        assert_eq!(header.to_string(), "##TSYNC_SIZE=1024##\n");
    }

    #[test]
    fn test_header_pieces_to_string() {
        let header = Header::Pieces(8);
        assert_eq!(header.to_string(), "##TSYNC_PIECES=8##\n");
    }

    #[test]
    fn test_header_name_to_string() {
        let header = Header::Name("aa.bin".to_string());
        assert_eq!(header.to_string(), "##TSYNC_NAME=aa.bin##\n");
    }
}
