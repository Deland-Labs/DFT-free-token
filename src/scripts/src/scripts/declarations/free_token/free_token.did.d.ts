import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ErrorInfo };
export interface ErrorInfo { 'code' : number, 'message' : string }
export type HistoryResult = { 'Ok' : Array<[string, ReceivesRewardRecord]> } |
  { 'Err' : ErrorInfo };
export type QuotaType = { 'LenEq' : number } |
  { 'LenGte' : number };
export interface ReceivesRewardRecord {
  'created_at' : bigint,
  'rewards' : Array<[RewardType, ReceivesRewardRecordState]>,
}
export type ReceivesRewardRecordState = { 'Sending' : null } |
  { 'Completed' : null };
export type RewardPackageResult = { 'Ok' : Array<RewardType> } |
  { 'Err' : ErrorInfo };
export type RewardPackagesResult = {
    'Ok' : Array<[string, Array<RewardType>]>
  } |
  { 'Err' : ErrorInfo };
export type RewardType = {
    'TokenTransferRewardPackage' : { 'canister' : Principal, 'amount' : bigint }
  } |
  { 'TokenMintRewardPackage' : { 'canister' : Principal, 'amount' : bigint } } |
  {
    'QuotaRewardPackage' : {
      'diff' : number,
      'canister' : Principal,
      'quota_type' : QuotaType,
    }
  };
export interface _SERVICE {
  'add_reward' : ActorMethod<
    [string, Array<RewardType>, [] | [Array<Principal>]],
    BooleanResult,
  >,
  'get_reward_package' : ActorMethod<[string], RewardPackageResult>,
  'get_reward_packages' : ActorMethod<[], RewardPackagesResult>,
  'history' : ActorMethod<[], HistoryResult>,
  'receive_free_token' : ActorMethod<[string], BooleanResult>,
}
