use crate::{
    msg::{QueryMsg, ResidualPaymentResponse},
    state::{
        Config, ResidualDefault, ResidualProtocol, ResidualProtocolKey, CONFIG, RESIDUAL_DEFAULT,
        ROYALTY_PROTOCOLS,
    },
};

use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use cw_utils::maybe_addr;
use terp_index_query::{QueryOptions, QueryOptionsInternal};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let api = deps.api;

    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::CollectionResidualDefault { collection } => to_json_binary(
            &query_collection_residual_default(deps, api.addr_validate(&collection)?)?,
        ),
        QueryMsg::CollectionResidualProtocol {
            collection,
            protocol,
        } => to_json_binary(&query_collection_residual_protocol(
            deps,
            api.addr_validate(&collection)?,
            api.addr_validate(&protocol)?,
        )?),
        QueryMsg::ResidualProtocolByCollection {
            collection,
            query_options,
        } => to_json_binary(&query_residual_protocol_by_collection(
            deps,
            api.addr_validate(&collection)?,
            query_options.unwrap_or_default(),
        )?),
        QueryMsg::ResidualPayment {
            collection,
            protocol,
        } => to_json_binary(&query_residual_payment(
            deps,
            api.addr_validate(&collection)?,
            maybe_addr(api, protocol)?,
        )?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

pub fn query_collection_residual_default(
    deps: Deps,
    collection: Addr,
) -> StdResult<Option<ResidualDefault>> {
    let residual_default = RESIDUAL_DEFAULT.may_load(deps.storage, collection)?;
    Ok(residual_default)
}

pub fn query_collection_residual_protocol(
    deps: Deps,
    collection: Addr,
    protocol: Addr,
) -> StdResult<Option<ResidualProtocol>> {
    let residual_protocol_key: ResidualProtocolKey = (collection, protocol);
    let residual_protocol = ROYALTY_PROTOCOLS.may_load(deps.storage, residual_protocol_key)?;
    Ok(residual_protocol)
}

pub fn query_residual_protocol_by_collection(
    deps: Deps,
    collection: Addr,
    query_options: QueryOptions<String>,
) -> StdResult<Vec<ResidualProtocol>> {
    let QueryOptionsInternal {
        limit,
        order,
        min,
        max,
    } = query_options.unpack(
        &Box::new(|sa: &String| Addr::unchecked(sa.clone())),
        None,
        None,
    );

    let residual_protocols: Vec<ResidualProtocol> = ROYALTY_PROTOCOLS
        .prefix(collection)
        .range(deps.storage, min, max, order)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;

    Ok(residual_protocols)
}

pub fn query_residual_payment(
    deps: Deps,
    collection: Addr,
    protocol: Option<Addr>,
) -> StdResult<ResidualPaymentResponse> {
    let residual_default = RESIDUAL_DEFAULT.may_load(deps.storage, collection.clone())?;

    let mut residual_protocol = None;
    if let Some(protocol_val) = &protocol {
        let residual_protocol_key: ResidualProtocolKey = (collection, protocol_val.clone());
        residual_protocol = ROYALTY_PROTOCOLS.may_load(deps.storage, residual_protocol_key)?;
    }

    Ok(ResidualPaymentResponse {
        residual_default,
        residual_protocol,
    })
}
