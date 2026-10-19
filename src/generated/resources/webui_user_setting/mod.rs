use crate::runtime;
use std::collections::BTreeMap;
pub mod delete;
pub mod index;
pub mod set;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn webui_user_setting(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn delete(
        &self,
        namespace: impl ToString,
        key: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("namespace".to_owned(), namespace.to_string());
        path_params.insert("key".to_owned(), key.to_string());
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
    pub fn set(
        &self,
        namespace: impl ToString,
        key: impl ToString,
    ) -> runtime::Request<'a, C, set::Input, set::Output, set::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("namespace".to_owned(), namespace.to_string());
        path_params.insert("key".to_owned(), key.to_string());
        runtime::Request::new(self.client, &set::SPEC, path_params, Default::default())
    }
    pub fn show(
        &self,
        namespace: impl ToString,
        key: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("namespace".to_owned(), namespace.to_string());
        path_params.insert("key".to_owned(), key.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
