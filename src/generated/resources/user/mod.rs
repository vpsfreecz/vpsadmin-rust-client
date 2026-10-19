use crate::runtime;
use std::collections::BTreeMap;
pub mod available_ips;
pub mod create;
pub mod current;
pub mod delete;
pub mod get_payment_instructions;
pub mod index;
pub mod show;
pub mod touch;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn user(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn available_ips(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, available_ips::Input, available_ips::Output, available_ips::Meta>
    {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(
            self.client,
            &available_ips::SPEC,
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
    pub fn current(
        &self,
    ) -> runtime::Request<'a, C, current::Input, current::Output, current::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(self.client, &current::SPEC, path_params, Default::default())
    }
    pub fn delete(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn get_payment_instructions(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        get_payment_instructions::Input,
        get_payment_instructions::Output,
        get_payment_instructions::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(
            self.client,
            &get_payment_instructions::SPEC,
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
    pub fn show(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn touch(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, touch::Input, touch::Output, touch::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(self.client, &touch::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        user_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("user_id".to_owned(), user_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
