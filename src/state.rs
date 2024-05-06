use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const CONFIG: Item<Config> = Item::new("config");
pub const SUM: Item<u64> = Item::new("sum");

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}
