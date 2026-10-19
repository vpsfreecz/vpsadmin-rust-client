use crate::runtime;
use std::collections::BTreeMap;
pub mod create;
pub mod delete;
pub mod index;
pub mod show;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn dns_server(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
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
    pub fn delete(
        &self,
        dns_server_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dns_server_id".to_owned(), dns_server_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(
        &self,
        dns_server_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dns_server_id".to_owned(), dns_server_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
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
    pub fn show(
        &self,
        dns_server_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dns_server_id".to_owned(), dns_server_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        dns_server_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dns_server_id".to_owned(), dns_server_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        dns_server_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dns_server_id".to_owned(), dns_server_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
