use crate::runtime;
use std::collections::BTreeMap;
pub mod array_top;
pub mod hash_top;
pub mod list_object_counts;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn debug(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn array_top(
        &self,
    ) -> runtime::Request<'a, C, array_top::Input, Vec<array_top::Output>, array_top::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &array_top::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn hash_top(
        &self,
    ) -> runtime::Request<'a, C, hash_top::Input, Vec<hash_top::Output>, hash_top::Meta> {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &hash_top::SPEC,
            path_params,
            Default::default(),
        )
    }
    pub fn list_object_counts(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        list_object_counts::Input,
        Vec<list_object_counts::Output>,
        list_object_counts::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &list_object_counts::SPEC,
            path_params,
            Default::default(),
        )
    }
}
