use crate::runtime;
use std::collections::BTreeMap;
pub mod create;
pub mod index;
pub mod set_maintenance;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn pool(&self) -> Handle<'_, C> {
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
    pub fn set_maintenance(
        &self,
        pool_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        set_maintenance::Input,
        set_maintenance::Output,
        set_maintenance::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("pool_id".to_owned(), pool_id.to_string());
        runtime::Request::new(
            self.client,
            &set_maintenance::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn show(
        &self,
        pool_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("pool_id".to_owned(), pool_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        pool_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("pool_id".to_owned(), pool_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
