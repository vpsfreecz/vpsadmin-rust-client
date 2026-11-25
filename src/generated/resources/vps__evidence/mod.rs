use crate::runtime;
use std::collections::BTreeMap;
pub mod index;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    // Double underscores preserve resource component boundaries in the public API.
    #[allow(non_snake_case)]
    pub fn vps__evidence(&self, vps_id: impl ToString) -> Handle<'_, C> {
        let path_params = BTreeMap::from([("vps_id".to_owned(), vps_id.to_string())]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn index(&self) -> runtime::Request<'a, C, index::Input, Vec<index::Output>, index::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &index::SPEC, path_params, Default::default())
    }
}
