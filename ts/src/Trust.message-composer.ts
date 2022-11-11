/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "cosmwasm";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { Uint128, InstantiateMsg, TrustScoreParams, ExecuteMsg, ReviewResult, QueryMsg, Addr, AccountsResponse, StakeAmountResponse, TrustInfoResponse, TrustInfo, TrustData } from "./Trust.types";
export interface TrustMessage {
  contractAddress: string;
  sender: string;
  updateConfig: ({
    admin,
    maintainer,
    maxRating,
    maxStakedDays,
    maxStakedTokens,
    reviewInterval,
    stakingContract,
    trustScoreParams
  }: {
    admin: string;
    maintainer: string;
    maxRating: number;
    maxStakedDays: number;
    maxStakedTokens: Uint128;
    reviewInterval: number;
    stakingContract: string;
    trustScoreParams: TrustScoreParams;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateStakingInfo: ({
    address
  }: {
    address: string;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
  review: ({
    address,
    result
  }: {
    address: string;
    result: ReviewResult;
  }, funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class TrustMessageComposer implements TrustMessage {
  sender: string;
  contractAddress: string;

  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateConfig = this.updateConfig.bind(this);
    this.updateStakingInfo = this.updateStakingInfo.bind(this);
    this.review = this.review.bind(this);
  }

  updateConfig = ({
    admin,
    maintainer,
    maxRating,
    maxStakedDays,
    maxStakedTokens,
    reviewInterval,
    stakingContract,
    trustScoreParams
  }: {
    admin: string;
    maintainer: string;
    maxRating: number;
    maxStakedDays: number;
    maxStakedTokens: Uint128;
    reviewInterval: number;
    stakingContract: string;
    trustScoreParams: TrustScoreParams;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_config: {
            admin,
            maintainer,
            max_rating: maxRating,
            max_staked_days: maxStakedDays,
            max_staked_tokens: maxStakedTokens,
            review_interval: reviewInterval,
            staking_contract: stakingContract,
            trust_score_params: trustScoreParams
          }
        })),
        funds
      })
    };
  };
  updateStakingInfo = ({
    address
  }: {
    address: string;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_staking_info: {
            address
          }
        })),
        funds
      })
    };
  };
  review = ({
    address,
    result
  }: {
    address: string;
    result: ReviewResult;
  }, funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          review: {
            address,
            result
          }
        })),
        funds
      })
    };
  };
}