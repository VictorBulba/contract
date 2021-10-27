#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage, to_binary};
use std::convert::{Infallible, TryInto};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, Infallible> {
    set_counter_value(deps.storage, 0).unwrap();
    Ok(Response::default())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    IncCounter {},
    SetCounter {
        new_value: u64,
    },
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, Infallible> {
    match msg {
        ExecuteMsg::IncCounter {} => {
            let current_value = get_counter_value(deps.storage);
            set_counter_value(deps.storage, current_value + 1)
        },
        ExecuteMsg::SetCounter { new_value } => set_counter_value(deps.storage, new_value),
    }
}

const COUNTER_KEY: &[u8] = b"COUNTER";

pub fn get_counter_value(storage: &dyn Storage) -> u64 {
    storage.get(COUNTER_KEY)
        .and_then(|vec| vec.try_into().ok())
        .map(|bytes| u64::from_le_bytes(bytes))
        .unwrap_or(0)
}

pub fn set_counter_value(storage: &mut dyn Storage, new_value: u64) -> Result<Response, Infallible> {
    let value = new_value.to_le_bytes();
    storage.set(COUNTER_KEY, &value);
    Ok(Response::default())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Messages {},
    CurrentCounterValue {},
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Messages {} => to_binary("Hello"),
        QueryMsg::CurrentCounterValue {} => to_binary(&get_counter_value(deps.storage)),
    }
}
