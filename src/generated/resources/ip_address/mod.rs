use crate::runtime;
use std::collections::BTreeMap;
pub mod assign;
pub mod assign_with_host_address;
pub mod create;
pub mod free;
pub mod index;
pub mod show;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn ip_address(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn assign(
        &self,
        ip_address_id: impl ToString,
    ) -> runtime::Request<'a, C, assign::Input, assign::Output, assign::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("ip_address_id".to_owned(), ip_address_id.to_string());
        runtime::Request::new(self.client, &assign::SPEC, path_params, Default::default())
    }
    pub fn assign_with_host_address(
        &self,
        ip_address_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        assign_with_host_address::Input,
        assign_with_host_address::Output,
        assign_with_host_address::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("ip_address_id".to_owned(), ip_address_id.to_string());
        runtime::Request::new(
            self.client,
            &assign_with_host_address::SPEC,
            path_params,
            Default::default(),
        )
    }
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
    pub fn free(
        &self,
        ip_address_id: impl ToString,
    ) -> runtime::Request<'a, C, free::Input, free::Output, free::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("ip_address_id".to_owned(), ip_address_id.to_string());
        runtime::Request::new(self.client, &free::SPEC, path_params, Default::default())
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
        ip_address_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("ip_address_id".to_owned(), ip_address_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        ip_address_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("ip_address_id".to_owned(), ip_address_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        ip_address_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("ip_address_id".to_owned(), ip_address_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
