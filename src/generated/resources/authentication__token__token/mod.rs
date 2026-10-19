use crate::runtime;
use std::collections::BTreeMap;
pub mod renew;
pub mod request;
pub mod reset_password;
pub mod revoke;
pub mod totp;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn authentication__token__token(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn renew(&self) -> runtime::Request<'a, C, renew::Input, renew::Output, renew::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &renew::SPEC, path_params, Default::default())
    }
    pub fn request(
        &self,
    ) -> runtime::Request<'a, C, request::Input, request::Output, request::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &request::SPEC, path_params, Default::default())
    }
    pub fn reset_password(
        &self,
    ) -> runtime::Request<'a, C, reset_password::Input, reset_password::Output, reset_password::Meta>
    {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &reset_password::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn revoke(&self) -> runtime::Request<'a, C, revoke::Input, revoke::Output, revoke::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &revoke::SPEC, path_params, Default::default())
    }
    pub fn totp(&self) -> runtime::Request<'a, C, totp::Input, totp::Output, totp::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &totp::SPEC, path_params, Default::default())
    }
}
