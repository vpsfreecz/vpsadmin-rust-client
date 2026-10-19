use crate::runtime;
use std::collections::BTreeMap;
pub mod create;
pub mod evacuate;
pub mod index;
pub mod overview_list;
pub mod public_status;
pub mod set_maintenance;
pub mod show;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn node(&self) -> Handle<'_, C> {
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
    pub fn evacuate(
        &self,
        node_id: impl ToString,
    ) -> runtime::Request<'a, C, evacuate::Input, evacuate::Output, evacuate::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("node_id".to_owned(), node_id.to_string());
        runtime::Request::new(
            self.client,
            &evacuate::SPEC,
            path_params,
            Default::default(),
        )
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
    pub fn overview_list(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        overview_list::Input,
        Vec<overview_list::Output>,
        overview_list::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &overview_list::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn public_status(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        public_status::Input,
        Vec<public_status::Output>,
        public_status::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &public_status::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn set_maintenance(
        &self,
        node_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        set_maintenance::Input,
        set_maintenance::Output,
        set_maintenance::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("node_id".to_owned(), node_id.to_string());
        runtime::Request::new(
            self.client,
            &set_maintenance::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn show(
        &self,
        node_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("node_id".to_owned(), node_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        node_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("node_id".to_owned(), node_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        node_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("node_id".to_owned(), node_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
