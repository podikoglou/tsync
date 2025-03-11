use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Header {
    Id(usize),
    Size(usize),
    Pieces(usize),
    Name(String),
    Checksum(u64),
}
