#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    pub trait IsTrustedApi<AccountId> where
		AccountId: codec::Codec,
     {
        fn is_trusted(account: AccountId, trustee: AccountId) -> bool;
    }
}
