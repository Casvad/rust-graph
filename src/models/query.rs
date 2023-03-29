use juniper::GraphQLObject;
use super::person::Person;

#[derive(GraphQLObject)]
pub(crate) struct QueryInternal {
    everyone: Vec<Person>,
}