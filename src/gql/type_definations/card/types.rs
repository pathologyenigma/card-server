use async_graphql::InputObject;
#[derive(InputObject)]
pub struct NewCard {
    name: String,
    description: String,
    logo: String,
}
