use crate::runtime;
use std::collections::BTreeMap;
pub mod index;
pub mod show;
pub mod update;
pub mod update_all;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    // Double underscores preserve resource component boundaries in the public API.
    #[allow(non_snake_case)]
    pub fn vps__feature(&self, vps_id: impl ToString) -> Handle<'_, C> {
        let path_params = BTreeMap::from([("vps_id".to_owned(), vps_id.to_string())]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
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
        feature_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("feature_id".to_owned(), feature_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        feature_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("feature_id".to_owned(), feature_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        feature_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("feature_id".to_owned(), feature_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
    pub fn update_all(
        &self,
    ) -> runtime::Request<'a, C, update_all::Input, update_all::Output, update_all::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &update_all::SPEC,
            path_params,
            Default::default(),
        )
    }
}
