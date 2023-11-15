use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consts::region::{RegionMap, REGIONS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    region: String,
}

#[derive(Error, Debug)]
pub enum RegionError {
    #[error("region not found")]
    NotFound,
}

impl Region {
    pub fn new(region: String) -> Result<Region> {
        for r in REGIONS {
            if r == &region {
                return Ok(Region { region });
            }
        }
        Err(RegionError::NotFound.into())
    }

    pub fn to_uint(&self) -> u8 {
        let region_map: RegionMap = RegionMap::new();
        let index: &usize = region_map.regions_name_to_index.get(&self.region).unwrap();
        *index as u8
    }
}

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.region)
    }
}

// FromStrを実装することで, from_str()で文字列から変換できるようになる
impl std::str::FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Region::new(s.to_string())
    }
}
