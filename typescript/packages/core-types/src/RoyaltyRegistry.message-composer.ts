/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.30.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Decimal, InstantiateMsg, Config, ExecuteMsg, QueryMsg, QueryBoundForString, QueryOptionsForString, NullableResidualDefault, Addr, Timestamp, Uint64, ResidualDefault, ResidualEntry, NullableResidualProtocol, ResidualProtocol, ResidualPaymentResponse, ArrayOfResidualProtocol } from "./ResidualRegistry.types";
export interface ResidualRegistryMessage {
  contractAddress: string;
  sender: string;
  initializeCollectionResidual: ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setCollectionResidualDefault: ({
    collection,
    recipient,
    share
  }: {
    collection: string;
    recipient: string;
    share: Decimal;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateCollectionResidualDefault: ({
    collection,
    decrement,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    recipient?: string;
    shareDelta?: Decimal;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setCollectionResidualProtocol: ({
    collection,
    protocol,
    recipient,
    share
  }: {
    collection: string;
    protocol: string;
    recipient: string;
    share: Decimal;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateCollectionResidualProtocol: ({
    collection,
    decrement,
    protocol,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    protocol: string;
    recipient?: string;
    shareDelta?: Decimal;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class ResidualRegistryMessageComposer implements ResidualRegistryMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.initializeCollectionResidual = this.initializeCollectionResidual.bind(this);
    this.setCollectionResidualDefault = this.setCollectionResidualDefault.bind(this);
    this.updateCollectionResidualDefault = this.updateCollectionResidualDefault.bind(this);
    this.setCollectionResidualProtocol = this.setCollectionResidualProtocol.bind(this);
    this.updateCollectionResidualProtocol = this.updateCollectionResidualProtocol.bind(this);
  }

  initializeCollectionResidual = ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          initialize_collection_residual: {
            collection
          }
        })),
        funds: _funds
      })
    };
  };
  setCollectionResidualDefault = ({
    collection,
    recipient,
    share
  }: {
    collection: string;
    recipient: string;
    share: Decimal;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_collection_residual_default: {
            collection,
            recipient,
            share
          }
        })),
        funds: _funds
      })
    };
  };
  updateCollectionResidualDefault = ({
    collection,
    decrement,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    recipient?: string;
    shareDelta?: Decimal;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_collection_residual_default: {
            collection,
            decrement,
            recipient,
            share_delta: shareDelta
          }
        })),
        funds: _funds
      })
    };
  };
  setCollectionResidualProtocol = ({
    collection,
    protocol,
    recipient,
    share
  }: {
    collection: string;
    protocol: string;
    recipient: string;
    share: Decimal;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_collection_residual_protocol: {
            collection,
            protocol,
            recipient,
            share
          }
        })),
        funds: _funds
      })
    };
  };
  updateCollectionResidualProtocol = ({
    collection,
    decrement,
    protocol,
    recipient,
    shareDelta
  }: {
    collection: string;
    decrement?: boolean;
    protocol: string;
    recipient?: string;
    shareDelta?: Decimal;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_collection_residual_protocol: {
            collection,
            decrement,
            protocol,
            recipient,
            share_delta: shareDelta
          }
        })),
        funds: _funds
      })
    };
  };
}