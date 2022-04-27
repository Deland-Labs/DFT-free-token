import type { Principal } from '@dfinity/principal';
export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ErrorInfo };
export interface ErrorInfo { 'code' : number, 'message' : string }
export interface _SERVICE {
  'receive_free_token' : () => Promise<BooleanResult>,
}
