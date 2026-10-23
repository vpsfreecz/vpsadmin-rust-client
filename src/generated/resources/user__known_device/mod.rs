use crate::runtime;
use std::collections::BTreeMap;
pub mod delete;
pub mod index;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    // Double underscores preserve resource component boundaries in the public API.
    #[allow(non_snake_case)]
    pub fn user__known_device(&self, user_id: impl ToString) -> Handle<'_, C> {
        let path_params = BTreeMap::from([("user_id".to_owned(), user_id.to_string())]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn delete(
        &self,
        known_device_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("known_device_id".to_owned(), known_device_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(
        &self,
        known_device_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("known_device_id".to_owned(), known_device_id.to_string());
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
        known_device_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("known_device_id".to_owned(), known_device_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        known_device_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("known_device_id".to_owned(), known_device_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
