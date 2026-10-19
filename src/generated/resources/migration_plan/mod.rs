use crate::runtime;
use std::collections::BTreeMap;
pub mod cancel;
pub mod create;
pub mod delete;
pub mod index;
pub mod show;
pub mod start;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn migration_plan(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn cancel(
        &self,
        migration_plan_id: impl ToString,
    ) -> runtime::Request<'a, C, cancel::Input, cancel::Output, cancel::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "migration_plan_id".to_owned(),
            migration_plan_id.to_string(),
        );
        runtime::Request::new(self.client, &cancel::SPEC, path_params, Default::default())
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
    pub fn delete(
        &self,
        migration_plan_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "migration_plan_id".to_owned(),
            migration_plan_id.to_string(),
        );
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(
        &self,
        migration_plan_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "migration_plan_id".to_owned(),
            migration_plan_id.to_string(),
        );
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
        migration_plan_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "migration_plan_id".to_owned(),
            migration_plan_id.to_string(),
        );
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        migration_plan_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "migration_plan_id".to_owned(),
            migration_plan_id.to_string(),
        );
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn start(
        &self,
        migration_plan_id: impl ToString,
    ) -> runtime::Request<'a, C, start::Input, start::Output, start::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "migration_plan_id".to_owned(),
            migration_plan_id.to_string(),
        );
        runtime::Request::new(self.client, &start::SPEC, path_params, Default::default())
    }
}
