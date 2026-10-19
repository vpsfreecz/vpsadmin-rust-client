use crate::runtime;
use std::collections::BTreeMap;
pub mod acknowledge;
pub mod ignore;
pub mod index;
pub mod show;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn monitored_event(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn acknowledge(
        &self,
        monitored_event_id: impl ToString,
    ) -> runtime::Request<'a, C, acknowledge::Input, acknowledge::Output, acknowledge::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "monitored_event_id".to_owned(),
            monitored_event_id.to_string(),
        );
        runtime::Request::new(
            self.client,
            &acknowledge::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn ack(
        &self,
        monitored_event_id: impl ToString,
    ) -> runtime::Request<'a, C, acknowledge::Input, acknowledge::Output, acknowledge::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "monitored_event_id".to_owned(),
            monitored_event_id.to_string(),
        );
        runtime::Request::new(
            self.client,
            &acknowledge::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn ignore(
        &self,
        monitored_event_id: impl ToString,
    ) -> runtime::Request<'a, C, ignore::Input, ignore::Output, ignore::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "monitored_event_id".to_owned(),
            monitored_event_id.to_string(),
        );
        runtime::Request::new(self.client, &ignore::SPEC, path_params, Default::default())
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
        monitored_event_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "monitored_event_id".to_owned(),
            monitored_event_id.to_string(),
        );
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        monitored_event_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert(
            "monitored_event_id".to_owned(),
            monitored_event_id.to_string(),
        );
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
