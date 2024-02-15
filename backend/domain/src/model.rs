use serde::{Serialize, Deserialize};

use self::{volunteer::VolunteerId, user_account::user_id::UserId};

pub mod apply;
pub mod condition;
pub mod gender;
pub mod group_account;
pub mod participant_account;
pub mod region;
pub mod scout;
pub mod target_status;
pub mod terms;
pub mod theme;
pub mod user_account;
pub mod volunteer;
pub mod group_participants;
pub mod favorite;
pub mod review;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewToId{
    Uid(UserId),
    Vid(VolunteerId)
}
