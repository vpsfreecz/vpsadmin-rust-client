use crate::runtime;
use std::collections::BTreeMap;
pub mod create;
pub mod delete;
pub mod disable_backups;
pub mod find_by_name;
pub mod index;
pub mod inherit;
pub mod migrate;
pub mod preview_snapshot_removal;
pub mod remove_snapshots;
pub mod show;
pub mod update;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn dataset(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
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
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn destroy(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, delete::Input, delete::Output, delete::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &delete::SPEC, path_params, Default::default())
    }
    pub fn disable_backups(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        disable_backups::Input,
        disable_backups::Output,
        disable_backups::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(
            self.client,
            &disable_backups::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn find_by_name(
        &self,
    ) -> runtime::Request<'a, C, find_by_name::Input, find_by_name::Output, find_by_name::Meta>
    {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &find_by_name::SPEC,
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
    pub fn inherit(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, inherit::Input, inherit::Output, inherit::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &inherit::SPEC, path_params, Default::default())
    }
    pub fn migrate(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, migrate::Input, migrate::Output, migrate::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &migrate::SPEC, path_params, Default::default())
    }
    pub fn preview_snapshot_removal(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        preview_snapshot_removal::Input,
        preview_snapshot_removal::Output,
        preview_snapshot_removal::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(
            self.client,
            &preview_snapshot_removal::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn remove_snapshots(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<
        'a,
        C,
        remove_snapshots::Input,
        remove_snapshots::Output,
        remove_snapshots::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(
            self.client,
            &remove_snapshots::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn show(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn find(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, show::Input, show::Output, show::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &show::SPEC, path_params, Default::default())
    }
    pub fn update(
        &self,
        dataset_id: impl ToString,
    ) -> runtime::Request<'a, C, update::Input, update::Output, update::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        path_params.insert("dataset_id".to_owned(), dataset_id.to_string());
        runtime::Request::new(self.client, &update::SPEC, path_params, Default::default())
    }
}
