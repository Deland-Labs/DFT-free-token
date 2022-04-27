export const idlFactory = ({ IDL }) => {
  const ErrorInfo = IDL.Record({ 'code' : IDL.Nat32, 'message' : IDL.Text });
  const BooleanResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : ErrorInfo });
  return IDL.Service({
    'receive_free_token' : IDL.Func([], [BooleanResult], []),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Principal, IDL.Nat, IDL.Opt(IDL.Vec(IDL.Principal))];
};
