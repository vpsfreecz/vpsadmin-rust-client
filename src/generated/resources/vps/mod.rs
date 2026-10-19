use crate::runtime;
use std::collections::BTreeMap;
pub mod boot;
pub mod clone;
pub mod create;
pub mod delete;
pub mod deploy_public_key;
pub mod index;
pub mod migrate;
pub mod passwd;
pub mod reinstall;
pub mod replace;
pub mod restart;
pub mod set_maintenance;
pub mod show;
pub mod start;
pub mod stop;
pub mod swap_with;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn vps(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn boot(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, boot::Input, boot::Output, boot::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &boot::SPEC, path_params, Default::default())
    }
    pub fn clone(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, clone::Input, clone::Output, clone::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &clone::SPEC, path_params, Default::default())
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
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn deploy_public_key(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        deploy_public_key::Input,
        deploy_public_key::Output,
        deploy_public_key::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(
            self.client,
            &deploy_public_key::SPEC,
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
    pub fn migrate(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, migrate::Input, migrate::Output, migrate::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &migrate::SPEC, path_params, Default::default())
    }
    pub fn passwd(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, passwd::Input, passwd::Output, passwd::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &passwd::SPEC, path_params, Default::default())
    }
    pub fn reinstall(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, reinstall::Input, reinstall::Output, reinstall::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(
            self.client,
            &reinstall::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn replace(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, replace::Input, replace::Output, replace::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &replace::SPEC, path_params, Default::default())
    }
    pub fn restart(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, restart::Input, restart::Output, restart::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &restart::SPEC, path_params, Default::default())
    }
    pub fn set_maintenance(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        set_maintenance::Input,
        set_maintenance::Output,
        set_maintenance::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(
            self.client,
            &set_maintenance::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn show(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn start(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, start::Input, start::Output, start::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &start::SPEC, path_params, Default::default())
    }
    pub fn stop(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, stop::Input, stop::Output, stop::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &stop::SPEC, path_params, Default::default())
    }
    pub fn swap_with(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, swap_with::Input, swap_with::Output, swap_with::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(
            self.client,
            &swap_with::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn update(
        &self,
        vps_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("vps_id".to_owned(), vps_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
