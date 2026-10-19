use crate::runtime;
use std::collections::BTreeMap;
pub mod candidates;
pub mod close;
pub mod create;
pub mod exempt;
pub mod index;
pub mod notify;
pub mod release;
pub mod show;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn ip_release_campaign(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn candidates(
        &self,
    ) -> runtime::Request<'a, C, candidates::Input, Vec<candidates::Output>, candidates::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &candidates::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn close(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, close::Input, close::Output, close::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &close::SPEC, path_params, Default::default())
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
    pub fn exempt(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, exempt::Input, exempt::Output, exempt::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &exempt::SPEC, path_params, Default::default())
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
    pub fn notify(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, notify::Input, notify::Output, notify::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &notify::SPEC, path_params, Default::default())
    }
    pub fn release(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, release::Input, release::Output, release::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &release::SPEC, path_params, Default::default())
    }
    pub fn show(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        ip_release_campaign_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "ip_release_campaign_id".to_owned(),
            ip_release_campaign_id.to_string(),
        );
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
