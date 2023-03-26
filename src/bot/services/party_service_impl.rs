use crate::core::services::party_service::PartyService;

#[derive(Clone)]
pub struct PartyServiceImpl;
impl PartyService for PartyServiceImpl {}

impl serenity::prelude::TypeMapKey for PartyServiceImpl {
    type Value = Self;
}