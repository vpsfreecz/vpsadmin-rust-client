use crate::runtime;
use std::collections::BTreeMap;
pub mod create;
pub mod delete;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    // Double underscores preserve resource component boundaries in the public API.
    #[allow(non_snake_case)]
    pub fn vps__console_token(&self, vps_id: impl ToString) -> Handle<'_, C> {
        let path_params = BTreeMap::from([("vps_id".to_owned(), vps_id.to_string())]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn create(&self) -> runtime::Request<'a, C, create::Input, create::Output, create::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &create::SPEC, path_params, Default::default())
    }
    pub fn new(&self) -> runtime::Request<'a, C, create::Input, create::Output, create::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &create::SPEC, path_params, Default::default())
    }
    pub fn delete(&self) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(&self) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn show(&self) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(&self) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
