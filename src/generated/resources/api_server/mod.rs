use crate::runtime;
use std::collections::BTreeMap;
pub mod unlock_transaction_signing_key;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn api_server(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn unlock_transaction_signing_key(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        unlock_transaction_signing_key::Input,
        unlock_transaction_signing_key::Output,
        unlock_transaction_signing_key::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &unlock_transaction_signing_key::SPEC,
            path_params,
            Default::default(),
        )
    }
}
