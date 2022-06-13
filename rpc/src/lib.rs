use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::Block as BlockT,
};

use std::sync::Arc;

pub use pallet_acuity_trusted_accounts_rpc_runtime_api::TrustedAccountsApiServer as TrustedAccountsRuntimeApi;

#[rpc]
pub trait TrustedAccountsApiServer<AccountId, BlockHash> {
	#[rpc(name = "trustedAccounts_isTrusted")]
	fn is_trusted(&self, account: AccountId, trustee: AccountId, at: Option<BlockHash>) -> Result<bool>;

	#[rpc(name = "trustedAccounts_isTrustedOnlyDeep")]
	fn is_trusted_only_deep(&self, account: AccountId, trustee: AccountId, at: Option<BlockHash>) -> Result<bool>;

	#[rpc(name = "trustedAccounts_isTrustedDeep")]
	fn is_trusted_deep(&self, account: AccountId, trustee: AccountId, at: Option<BlockHash>) -> Result<bool>;

	#[rpc(name = "trustedAccounts_trustedBy")]
	fn trusted_by(&self, account: AccountId, at: Option<BlockHash>) -> Result<Vec<AccountId>>;

	#[rpc(name = "trustedAccounts_trustedByThatTrust")]
	fn trusted_by_that_trust(&self, account: AccountId, account_is_trusted_by_trusted: AccountId, at: Option<BlockHash>) -> Result<Vec<AccountId>>;
}

pub struct TrustedAccounts<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> TrustedAccounts<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, AccountId, Block> TrustedAccountsApiServer<AccountId, <Block as BlockT>::Hash>
	for TrustedAccounts<C, Block>
where
    AccountId: Codec,
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: TrustedAccountsRuntimeApi<Block, AccountId>,
{
    fn is_trusted(
        &self,
        account: AccountId,
        trustee: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {

    	let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

    	api.is_trusted(&at, account, trustee).map_err(|e| Error {
    		code: ErrorCode::ServerError(1234),
    		message: "Unable to query dispatch info.".into(),
    		data: Some(e.to_string().into()),
    	})
	}

	fn is_trusted_only_deep(
        &self,
        account: AccountId,
        trustee: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {

    	let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

    	api.is_trusted_only_deep(&at, account, trustee).map_err(|e| Error {
    		code: ErrorCode::ServerError(1234),
    		message: "Unable to query dispatch info.".into(),
    		data: Some(e.to_string().into()),
    	})
	}

	fn is_trusted_deep(
        &self,
        account: AccountId,
        trustee: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {

    	let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

    	api.is_trusted_deep(&at, account, trustee).map_err(|e| Error {
    		code: ErrorCode::ServerError(1234),
    		message: "Unable to query dispatch info.".into(),
    		data: Some(e.to_string().into()),
    	})
	}

	fn trusted_by(
        &self,
        account: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<AccountId>> {

    	let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

    	api.trusted_by(&at, account).map_err(|e| Error {
    		code: ErrorCode::ServerError(1234),
    		message: "Unable to query dispatch info.".into(),
    		data: Some(e.to_string().into()),
    	})
	}

	fn trusted_by_that_trust(
        &self,
        account: AccountId,
        account_is_trusted_by_trusted: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<AccountId>> {

    	let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

    	api.trusted_by_that_trust(&at, account, account_is_trusted_by_trusted).map_err(|e| Error {
    		code: ErrorCode::ServerError(1234),
    		message: "Unable to query dispatch info.".into(),
    		data: Some(e.to_string().into()),
    	})
	}
}
