use crate::runtime;
use std::collections::BTreeMap;
pub mod index;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    // Double underscores preserve resource component boundaries in the public API.
    #[allow(non_snake_case)]
    pub fn ip_release_campaign__attempt(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> Handle<'_, C> {
        let path_params = BTreeMap::from([(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        )]);
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
        attempt_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("attempt_id".to_owned(), attempt_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        attempt_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("attempt_id".to_owned(), attempt_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
