use crate::runtime;
use std::collections::BTreeMap;
pub mod estimate_income;

pub struct Handle<'a, C: runtime::Mode> {
    client: &'a runtime::Client<C>,
    path_params: BTreeMap<String, String>,
}
impl<C: runtime::Mode> runtime::Client<C> {
    pub fn payment_stats(&self) -> Handle<'_, C> {
        let path_params = BTreeMap::from([]);
        Handle {
            client: self,
            path_params,
        }
    }
}
impl<'a, C: runtime::Mode> Handle<'a, C> {
    pub fn estimate_income(
        &self,
    ) -> runtime::Request<
        'a,
        C,
        estimate_income::Input,
        estimate_income::Output,
        estimate_income::Meta,
    > {
        #[allow(unused_mut)]
        let mut path_params = self.path_params.clone();
        runtime::Request::new(
            self.client,
            &estimate_income::SPEC,
            path_params,
            Default::default(),
        )
    }
}
