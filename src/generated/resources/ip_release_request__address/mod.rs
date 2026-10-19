use crate::runtime;
use std::collections::BTreeMap;
pub mod exempt;
pub mod index;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn ip_release_request__address(
        &self,
        ip_release_request_id: impl ToString,
    ) -> Handle<'_, C> {
        let path_params = BTreeMap::from([(
            "ip_release_request_id".to_owned(),
            ip_release_request_id.to_string(),
        )]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn exempt(
        &self,
        address_id: impl ToString,
    ) -> runtime::Request<'a, C, exempt::Input, exempt::Output, exempt::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("address_id".to_owned(), address_id.to_string());
        runtime::Request::new(self.client, &exempt::SPEC, path_params, Default::default())
    }
    pub fn index(&self) -> runtime::Request<'a, C, index::Input, Vec<index::Output>, index::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &index::SPEC, path_params, Default::default())
    }
    pub fn list(&self) -> runtime::Request<'a, C, index::Input, Vec<index::Output>, index::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &index::SPEC, path_params, Default::default())
    }
}
