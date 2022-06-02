import type { Principal } from '@dfinity/principal';
export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ErrorInfo };
export interface ErrorInfo { 'code' : number, 'message' : string }
export type QuotaType = { 'LenEq' : number } |
  { 'LenGte' : number };
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
  'add_reward' : (
      arg_0: string,
      arg_1: Array<RewardType>,
      arg_2: [] | [Array<Principal>],
    ) => Promise<BooleanResult>,
  'receive_free_token' : (arg_0: string) => Promise<BooleanResult>,
}
