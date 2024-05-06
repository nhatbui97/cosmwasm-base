#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, SUM};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // save config to storage
    CONFIG.save(deps.storage, &Config { owner: msg.owner })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Example { a, b } => execute_example(deps, info, a, b),
    }
}

fn execute_example(
    deps: DepsMut,
    info: MessageInfo,
    a: u64,
    b: u64,
) -> Result<Response, ContractError> {
    // load config from storage
    let config = CONFIG.load(deps.storage)?;

    // check owner: only contract owner can execute
    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let sum = a + b;
    //save sum to storage
    SUM.save(deps.storage, &sum)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // read SUM from storage
        QueryMsg::Example {} => to_json_binary(&SUM.load(deps.storage)?),
    }
}

#[cfg(test)]
mod tests {}
