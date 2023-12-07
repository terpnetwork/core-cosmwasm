use crate::state::Extension;
use cosmwasm_schema::cw_serde;
use factory_utils::msg::{CreateMinterMsg, FactoryUtilsExecuteMsg, UpdateMinterParamsMsg};

use crate::state::BaseMinterParams;

#[cw_serde]
pub struct InstantiateMsg {
    pub params: BaseMinterParams,
}

pub type BaseMinterCreateMsg = CreateMinterMsg<Extension>;

pub type ExecuteMsg = FactoryUtilsExecuteMsg<Extension>;

pub type BaseUpdateParamsMsg = UpdateMinterParamsMsg<Extension>;

#[cw_serde]
pub enum SudoMsg<T> {
    UpdateParams(Box<T>),
}
pub type BaseSudoMsg = SudoMsg<BaseUpdateParamsMsg>;

#[cw_serde]
pub struct ParamsResponse {
    pub params: BaseMinterParams,
}
