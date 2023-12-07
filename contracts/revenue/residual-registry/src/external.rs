use crate::{
    msg::{ExecuteMsg, QueryMsg, ResidualPaymentResponse},
    state::ResidualEntry,
    ContractError,
};

use cosmwasm_std::{ensure, to_json_binary, Addr, Deps, MessageInfo, QuerierWrapper, WasmMsg};
use terp721_base::msg::{CollectionInfoResponse, QueryMsg as Terp721QueryMsg};
use terp_sdk::Response;

/// Ensures that the sender is the collection creator.
pub fn only_collection_creator(
    deps: Deps,
    info: &MessageInfo,
    collection: &Addr,
) -> Result<(), ContractError> {
    let collection_info: CollectionInfoResponse = deps
        .querier
        .query_wasm_smart(collection, &Terp721QueryMsg::CollectionInfo {})?;

    ensure!(
        info.sender == collection_info.creator,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
    );

    Ok(())
}

/// Invoke `fetch_residual_entry` to fetch the residuals for a given NFT sale
/// with an optional protocol address.
///
/// # Arguments
///
/// * `deps` - [cosmwasm_std::Deps]
/// * `residual_registry` - The address of the residual registry.
/// * `collection` - The address of the collection contract to fetch residuals for.
/// * `protocol` - The address of the protocol looking to pay residuals (optional).
///
/// # Returns
///
/// * `ResidualEntry` - The [ResidualEntry] for the given collection and protocol (if any).
///
pub fn fetch_residual_entry(
    querier: &QuerierWrapper,
    residual_registry: &Addr,
    collection: &Addr,
    protocol: Option<&Addr>,
) -> Result<Option<ResidualEntry>, ContractError> {
    let residual_payment_response = querier.query_wasm_smart::<ResidualPaymentResponse>(
        residual_registry,
        &QueryMsg::ResidualPayment {
            collection: collection.to_string(),
            protocol: protocol.map(|p| p.to_string()),
        },
    )?;

    if let Some(residual_protocol) = residual_payment_response.residual_protocol {
        return Ok(Some(residual_protocol.residual_entry));
    }

    if let Some(residual_default) = residual_payment_response.residual_default {
        return Ok(Some(residual_default.residual_entry));
    }

    Ok(None)
}

/// Invoke `fetch_or_set_residuals` to fetch the residuals for a given NFT sale
/// with an optional protocol address. If residuals are not found on the residual registry
/// then the collection contract's residuals are used, and the collection contract's residuals
/// are set on the residual registry.
///
/// # Arguments
///
/// * `deps` - [cosmwasm_std::Deps]
/// * `residual_registry` - The address of the residual registry.
/// * `collection` - The address of the collection contract to fetch residuals for.
/// * `protocol` - The address of the protocol looking to pay residuals (optional).
/// * `response` - The [cosmwasm_std::Response] object used to append the message.
///
/// # Returns
///
/// * `ResidualEntry` - The [ResidualEntry] for the given collection and protocol (if any).
/// * `Response` - The [cosmwasm_std::Response] with the appended message.
///
pub fn fetch_or_set_residuals(
    deps: Deps,
    residual_registry: &Addr,
    collection: &Addr,
    protocol: Option<&Addr>,
    mut response: Response,
) -> Result<(Option<ResidualEntry>, Response), ContractError> {
    let residual_entry = fetch_residual_entry(&deps.querier, residual_registry, collection, protocol)?;
    if let Some(residual_entry) = residual_entry {
        return Ok((Some(residual_entry), response));
    }

    let collection_info: CollectionInfoResponse = deps
        .querier
        .query_wasm_smart(collection, &Terp721QueryMsg::CollectionInfo {})?;

    if let Some(residual_info_response) = collection_info.residual_info {
        let residual_entry = ResidualEntry {
            recipient: deps
                .api
                .addr_validate(&residual_info_response.payment_address)?,
            share: residual_info_response.share,
            updated: None,
        };

        response = response.add_message(WasmMsg::Execute {
            contract_addr: residual_registry.to_string(),
            msg: to_json_binary(&ExecuteMsg::InitializeCollectionResidual {
                collection: collection.to_string(),
            })
            .unwrap(),
            funds: vec![],
        });

        return Ok((Some(residual_entry), response));
    }

    Ok((None, response))
}
