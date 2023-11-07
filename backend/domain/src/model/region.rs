use serde::{Deserialize, Serialize};

use crate::consts::region::{RegionMap, REGIONS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    region: String,
}

#[derive(Debug)]
pub struct RegionNotFoundError;

impl Region {
    pub fn new(region: String) -> Result<Region, RegionNotFoundError> {
        for r in REGIONS {
            if r == &region {
                return Ok(Region { region });
            }
        }
        Err(RegionNotFoundError)
    }

    pub fn to_uint(&self) -> u8 {
        let region_map: RegionMap = RegionMap::new();
        let index: &usize = region_map.regions_name_to_index.get(&self.region).unwrap();
        *index as u8
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.region)
    }
}
