use juniper::{EmptyMutation, OperationType, RootNode};
use crate::models::person::Person;
use crate::models::query::QueryInternal;


fn build_schema() {
    RootNode::new(QueryInternal, EmptyMutation::<()>::new(), ());
}


