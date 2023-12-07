use crate::{
    error::ContractError,
    external::only_collection_creator,
    msg::ExecuteMsg,
    state::{
        ResidualDefault, ResidualEntry, ResidualProtocol, ResidualProtocolKey, CONFIG,
        RESIDUAL_DEFAULT, ROYALTY_PROTOCOLS,
    },
};

use cosmwasm_std::{attr, ensure, Addr, Decimal, DepsMut, Env, Event, MessageInfo};
use cw_utils::{maybe_addr, nonpayable};
use terp721_base::msg::{CollectionInfoResponse, QueryMsg as Terp721QueryMsg};
use terp_sdk::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        ExecuteMsg::InitializeCollectionResidual { collection } => {
            execute_initialize_collection_residual(deps, info, env, api.addr_validate(&collection)?)
        }
        ExecuteMsg::SetCollectionResidualDefault {
            collection,
            recipient,
            share,
        } => execute_set_collection_residual_default(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            api.addr_validate(&recipient)?,
            share,
        ),
        ExecuteMsg::UpdateCollectionResidualDefault {
            collection,
            recipient,
            share_delta,
            decrement,
        } => execute_update_collection_residual_default(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            maybe_addr(api, recipient)?,
            share_delta,
            decrement,
        ),
        ExecuteMsg::SetCollectionResidualProtocol {
            collection,
            protocol,
            recipient,
            share,
        } => execute_set_collection_residual_protocol(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            api.addr_validate(&protocol)?,
            api.addr_validate(&recipient)?,
            share,
        ),
        ExecuteMsg::UpdateCollectionResidualProtocol {
            collection,
            protocol,
            recipient,
            share_delta,
            decrement,
        } => execute_update_collection_residual_protocol(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            api.addr_validate(&protocol)?,
            maybe_addr(api, recipient)?,
            share_delta,
            decrement,
        ),
    }
}

pub fn execute_initialize_collection_residual(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut response = Response::new();

    let residual_default = RESIDUAL_DEFAULT.may_load(deps.storage, collection.clone())?;
    ensure!(
        residual_default.is_none(),
        ContractError::InvalidCollectionResidual(
            "Collection residual already initialized".to_string()
        )
    );

    let collection_info: CollectionInfoResponse = deps
        .querier
        .query_wasm_smart(collection.clone(), &Terp721QueryMsg::CollectionInfo {})?;

    if let Some(residual_info) = collection_info.residual_info {
        let residual_entry = ResidualEntry {
            recipient: deps.api.addr_validate(&residual_info.payment_address)?,
            share: residual_info.share,
            updated: None,
        };

        residual_entry.validate()?;

        RESIDUAL_DEFAULT.save(
            deps.storage,
            collection.clone(),
            &ResidualDefault {
                collection: collection.clone(),
                residual_entry,
            },
        )?;

        response = response.add_event(Event::new("initialize-collection-residual").add_attributes(
            vec![
                attr("collection", collection.to_string()),
                attr("recipient", residual_info.payment_address.to_string()),
                attr("share", residual_info.share.to_string()),
                attr("updated", env.block.time.to_string()),
            ],
        ));
    } else {
        return Err(ContractError::InvalidCollectionResidual(
            "Collection contract residuals not found".to_string(),
        ));
    }

    Ok(response)
}

pub fn execute_set_collection_residual_default(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    recipient: Addr,
    share: Decimal,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let mut response = Response::new();

    let residual_default = RESIDUAL_DEFAULT.may_load(deps.storage, collection.clone())?;
    ensure!(
        residual_default.is_none(),
        ContractError::InvalidCollectionResidual(
            "Collection residual already initialized".to_string()
        )
    );

    let residual_default = ResidualDefault {
        collection: collection.clone(),
        residual_entry: ResidualEntry {
            recipient: recipient.clone(),
            share,
            updated: Some(env.block.time),
        },
    };

    residual_default.residual_entry.validate()?;

    RESIDUAL_DEFAULT.save(deps.storage, collection.clone(), &residual_default)?;

    response = response.add_event(Event::new("set-collection-residual-default").add_attributes(
        vec![
            attr("collection", collection.to_string()),
            attr("recipient", recipient.to_string()),
            attr("share", share.to_string()),
            attr("updated", env.block.time.to_string()),
        ],
    ));

    Ok(response)
}

pub fn execute_update_collection_residual_default(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    recipient: Option<Addr>,
    share_delta: Option<Decimal>,
    decrement: Option<bool>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let config = CONFIG.load(deps.storage)?;
    let mut response = Response::new();

    let mut residual_default = RESIDUAL_DEFAULT
        .load(deps.storage, collection.clone())
        .map_err(|_| {
            ContractError::InvalidCollectionResidual("Collection residual does not exist".to_string())
        })?;

    if let Some(updated) = residual_default.residual_entry.updated {
        ensure!(
            updated.plus_seconds(config.update_wait_period) <= env.block.time,
            ContractError::Unauthorized("Residual entry cannot be updated yet".to_string())
        );
    }

    let mut event = Event::new("update-collection-residual-default")
        .add_attribute("collection", collection.to_string());

    if let Some(recipient) = recipient {
        residual_default.residual_entry.recipient = recipient.clone();
        event = event.add_attribute("recipient", recipient.to_string());
    }

    if let Some(share_delta) = share_delta {
        residual_default
            .residual_entry
            .update_share(&config, share_delta, decrement)?;

        event = event.add_attribute("share", residual_default.residual_entry.share.to_string());
    }

    residual_default.residual_entry.updated = Some(env.block.time);
    residual_default.residual_entry.validate()?;
    RESIDUAL_DEFAULT.save(deps.storage, collection, &residual_default)?;

    response = response.add_event(event);

    Ok(response)
}

pub fn execute_set_collection_residual_protocol(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    protocol: Addr,
    recipient: Addr,
    share: Decimal,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let mut response = Response::new();

    let residual_protocol_key: ResidualProtocolKey = (collection.clone(), protocol.clone());
    let residual_protocol =
        ROYALTY_PROTOCOLS.may_load(deps.storage, residual_protocol_key.clone())?;
    if residual_protocol.is_some() {
        return Err(ContractError::InvalidCollectionResidual(
            "Collection residual protocol already initialized".to_string(),
        ));
    }

    let residual_entry = ResidualEntry {
        recipient: recipient.clone(),
        share,
        updated: Some(env.block.time),
    };
    residual_entry.validate()?;
    ROYALTY_PROTOCOLS.save(
        deps.storage,
        residual_protocol_key,
        &ResidualProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            residual_entry,
        },
    )?;

    response = response.add_event(
        Event::new("set-collection-residual-protocol").add_attributes(vec![
            attr("collection", collection.to_string()),
            attr("protocol", protocol.to_string()),
            attr("recipient", recipient.to_string()),
            attr("share", share.to_string()),
            attr("updated", env.block.time.to_string()),
        ]),
    );

    Ok(response)
}

#[allow(clippy::too_many_arguments)]
pub fn execute_update_collection_residual_protocol(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    protocol: Addr,
    recipient: Option<Addr>,
    share_delta: Option<Decimal>,
    decrement: Option<bool>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let config = CONFIG.load(deps.storage)?;
    let mut response = Response::new();

    let residual_protocol_key: ResidualProtocolKey = (collection.clone(), protocol);
    let mut residual_protocol = ROYALTY_PROTOCOLS
        .load(deps.storage, residual_protocol_key.clone())
        .map_err(|_| {
            ContractError::InvalidCollectionResidual("Collection residual does not exist".to_string())
        })?;

    if let Some(updated) = residual_protocol.residual_entry.updated {
        ensure!(
            updated.plus_seconds(config.update_wait_period) <= env.block.time,
            ContractError::Unauthorized("Residual entry cannot be updated yet".to_string())
        );
    }

    let mut event = Event::new("update-collection-residual-protocol")
        .add_attribute("collection", collection.to_string());

    if let Some(recipient) = recipient {
        residual_protocol.residual_entry.recipient = recipient.clone();
        event = event.add_attribute("recipient", recipient.to_string());
    }

    if let Some(share_delta) = share_delta {
        residual_protocol
            .residual_entry
            .update_share(&config, share_delta, decrement)?;
        event = event.add_attribute("share", residual_protocol.residual_entry.share.to_string());
    }

    residual_protocol.residual_entry.updated = Some(env.block.time);
    residual_protocol.residual_entry.validate()?;
    ROYALTY_PROTOCOLS.save(deps.storage, residual_protocol_key, &residual_protocol)?;

    response = response.add_event(event);

    Ok(response)
}
