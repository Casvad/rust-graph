use juniper::GraphQLObject;
use serde::{Serialize, Deserialize};

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
///A person in our system
pub(crate) struct Person {
    ///Autogenerated identification for the user
    pub(crate) uuid: String,
    ///The email used to register
    #[deprecated(note = "Use `emails`")]
    email: String,
    ///The emails of the user
    emails: Vec<String>,
    /// The age of the person
    age: i32,
    #[graphql(skip)]
    password_hash: String,
}

impl Person {
    fn email(&self) -> String {
        self.email
    }

    fn age(&self) -> i32 {
        self.age
    }

    fn can_vote(&self) -> bool {
        self.age >= 18
    }
}