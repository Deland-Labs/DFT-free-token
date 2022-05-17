export const idlFactory = ({ IDL }) => {
  const QuotaType = IDL.Variant({ 'LenEq' : IDL.Nat8, 'LenGte' : IDL.Nat8 });
  const RewardType = IDL.Variant({
    'TokenTransferRewardPackage' : IDL.Record({
      'canister' : IDL.Principal,
      'amount' : IDL.Nat,
    }),
    'TokenMintRewardPackage' : IDL.Record({
      'canister' : IDL.Principal,
      'amount' : IDL.Nat,
    }),
    'QuotaRewardPackage' : IDL.Record({
      'diff' : IDL.Nat32,
      'canister' : IDL.Principal,
      'quota_type' : QuotaType,
    }),
  });
  const ErrorInfo = IDL.Record({ 'code' : IDL.Nat32, 'message' : IDL.Text });
  const BooleanResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : ErrorInfo });
  return IDL.Service({
    'add_reward' : IDL.Func(
        [IDL.Text, IDL.Vec(RewardType), IDL.Opt(IDL.Vec(IDL.Principal))],
        [BooleanResult],
        [],
      ),
    'receive_free_token' : IDL.Func([IDL.Text], [BooleanResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
