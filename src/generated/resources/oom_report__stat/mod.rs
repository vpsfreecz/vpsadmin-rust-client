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
    pub fn oom_report__stat(&self, oom_report_id: impl ToString) -> Handle<'_, C> {
        let path_params = BTreeMap::from([("oom_report_id".to_owned(), oom_report_id.to_string())]);
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
        stat_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("stat_id".to_owned(), stat_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        stat_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("stat_id".to_owned(), stat_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
}
