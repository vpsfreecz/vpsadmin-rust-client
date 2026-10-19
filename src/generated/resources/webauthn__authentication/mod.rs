use crate::runtime;
use std::collections::BTreeMap;
pub mod begin;
pub mod finish;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn webauthn__authentication(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn begin(&self) -> runtime::Request<'a, C, begin::Input, begin::Output, begin::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &begin::SPEC, path_params, Default::default())
    }
    pub fn finish(&self) -> runtime::Request<'a, C, finish::Input, finish::Output, finish::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &finish::SPEC, path_params, Default::default())
    }
}
