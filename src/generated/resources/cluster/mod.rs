use crate::runtime;
use std::collections::BTreeMap;
pub mod full_stats;
pub mod generate_migration_keys;
pub mod public_stats;
pub mod search;
pub mod set_maintenance;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn cluster(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn full_stats(
        &self,
    ) -> runtime::Request<'a, C, full_stats::Input, full_stats::Output, full_stats::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &full_stats::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn generate_migration_keys(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        generate_migration_keys::Input,
        generate_migration_keys::Output,
        generate_migration_keys::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &generate_migration_keys::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn public_stats(
        &self,
    ) -> runtime::Request<'a, C, public_stats::Input, public_stats::Output, public_stats::Meta>
    {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &public_stats::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn search(
        &self,
    ) -> runtime::Request<'a, C, search::Input, Vec<search::Output>, search::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &search::SPEC, path_params, Default::default())
    }
    pub fn set_maintenance(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        set_maintenance::Input,
        set_maintenance::Output,
        set_maintenance::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &set_maintenance::SPEC,
            path_params,
            Default::default(),
        )
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
