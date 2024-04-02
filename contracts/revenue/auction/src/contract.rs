use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
};
use cw721::Cw721ReceiveMsg;
use terp_marketplace_utils::auction::{Cw721HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::auction::{
    admin_cancel_auction, admin_change_config, admin_pause, admin_resume, cancel_auction,
    create_auction, place_bid, set_royalty_admin, set_royalty_fee, settle_auction, settle_hook,
};
use crate::error::ContractError;
use crate::querier::{
    construct_action_response, query_all_royalty, query_auction, query_auction_by_amount,
    query_auction_by_bidder, query_auction_by_end_time, query_auction_by_nft,
    query_auction_by_seller, query_bid_history_by_auction_id, query_bid_number,
    query_calculate_price, query_config, query_nft_auction_map, query_not_started_auctions,
    query_royalty_admin, query_royalty_fee, query_state,
};
use crate::state::{Config, State, CONFIG, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
        protocol_fee: msg.protocol_fee,
        min_reserve_price: msg.min_reserve_price,
        max_royalty_fee: msg.max_royalty_fee,
        duration: msg.duration,
        min_duration: msg.min_duration,
        min_increment: msg.min_increment,
        accepted_denom: msg.accepted_denom,
        protocol_addr: deps.api.addr_validate(&msg.protocol_addr)?,
    };

    CONFIG.save(deps.storage, &config)?;

    let state = State {
        next_auction_id: Uint128::zero(),
        is_freeze: false,
    };

    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveNft(msg) => receive_nft(deps, env, info, msg),
        ExecuteMsg::PlaceBid { auction_id } => place_bid(deps, env, info, auction_id),
        ExecuteMsg::Settle { auction_id } => settle_auction(deps, env, auction_id),
        ExecuteMsg::CancelAuction { auction_id } => cancel_auction(deps, env, info, auction_id),
        ExecuteMsg::AdminCancelAuction { auction_id } => {
            admin_cancel_auction(deps, env, info, auction_id)
        }
        ExecuteMsg::AdminPause {} => admin_pause(deps, env, info),
        ExecuteMsg::AdminResume {} => admin_resume(deps, env, info),
        ExecuteMsg::AdminChangeConfig {
            protocol_fee,
            min_increment,
            min_reserve_price,
            max_royalty_fee,
            duration,
            min_duration,
            accepted_denom,
            protocol_addr,
        } => admin_change_config(
            deps,
            env,
            info,
            protocol_fee,
            min_increment,
            min_reserve_price,
            max_royalty_fee,
            duration,
            min_duration,
            accepted_denom,
            protocol_addr,
        ),
        ExecuteMsg::SetRoyaltyFee {
            contract_addr,
            royalty_fee,
            creator,
        } => set_royalty_fee(deps, env, info, contract_addr, creator, royalty_fee),
        ExecuteMsg::SetRoyaltyAdmin { address, enable } => {
            set_royalty_admin(deps, env, info, address, enable)
        }
        ExecuteMsg::SettleHook {
            nft_contract,
            token_id,
            owner,
        } => settle_hook(deps, env, info, nft_contract, token_id, owner),
    }
}

pub fn receive_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw721_msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&cw721_msg.msg) {
        Ok(Cw721HookMsg::CreateAuction {
            denom,
            reserve_price,
            is_instant_sale,
        }) => {
            // need to check that this contract is owner of nft to prevent malicious contract call this function directly
            let seller = deps.api.addr_validate(&cw721_msg.sender)?;
            let nft_contract = info.sender.clone();
            let token_id = cw721_msg.token_id.clone();
            create_auction(
                deps,
                env,
                nft_contract,
                token_id.clone(),
                seller,
                denom,
                reserve_price,
                is_instant_sale,
            )
        }
        Err(err) => Err(ContractError::Std(StdError::generic_err(err.to_string()))),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::State {} => to_json_binary(&query_state(deps)?),
        QueryMsg::Auction { auction_id } => to_json_binary(&query_auction(deps, auction_id)?),
        QueryMsg::RoyaltyFee { contract_addr } => {
            to_json_binary(&query_royalty_fee(deps, contract_addr)?)
        }
        QueryMsg::RoyaltyAdmin { address } => to_json_binary(&query_royalty_admin(deps, address)?),
        QueryMsg::AllRoyaltyFee { start_after, limit } => {
            to_json_binary(&query_all_royalty(deps, start_after, limit)?)
        }
        QueryMsg::CalculatePrice {
            nft_contract,
            token_id,
            amount,
        } => to_json_binary(&query_calculate_price(
            deps,
            nft_contract,
            token_id,
            amount,
        )?),
        QueryMsg::NftAuction {
            nft_contract,
            token_id,
        } => to_json_binary(&query_nft_auction_map(deps, nft_contract, token_id)?),
        QueryMsg::BidHistoryByAuctionId { auction_id, limit } => {
            to_json_binary(&query_bid_history_by_auction_id(deps, auction_id, limit)?)
        }
        QueryMsg::BidsCount { auction_id } => to_json_binary(&query_bid_number(deps, auction_id)?),
        QueryMsg::AuctionByContract {
            nft_contract,
            limit,
        } => {
            let auction_ids = query_auction_by_nft(deps, nft_contract, limit)?;
            to_json_binary(&construct_action_response(deps, auction_ids)?)
        }
        QueryMsg::AuctionBySeller { seller, limit } => {
            let auction_ids = query_auction_by_seller(deps, seller, limit)?;
            to_json_binary(&construct_action_response(deps, auction_ids)?)
        }
        QueryMsg::AuctionByEndTime {
            nft_contract,
            end_time,
            limit,
            is_desc,
        } => {
            let auction_ids =
                query_auction_by_end_time(deps, nft_contract, end_time, limit, is_desc)?;
            to_json_binary(&construct_action_response(deps, auction_ids)?)
        }
        QueryMsg::AuctionByAmount {
            nft_contract,
            amount,
            limit,
        } => {
            let auction_ids = query_auction_by_amount(deps, nft_contract, amount, limit)?;
            to_json_binary(&construct_action_response(deps, auction_ids)?)
        }
        QueryMsg::NotStartedAuction {
            nft_contract,
            start_after,
            limit,
            is_desc,
        } => {
            let auction_ids =
                query_not_started_auctions(deps, nft_contract, start_after, limit, is_desc)?;
            to_json_binary(&construct_action_response(deps, auction_ids)?)
        }
        QueryMsg::AuctionByBidder {
            bidder,
            start_after,
            limit,
        } => {
            let auction_ids = query_auction_by_bidder(deps, bidder, start_after, limit)?;
            to_json_binary(&construct_action_response(deps, auction_ids)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
