/// just for graphql input obejct to database model
/// the args could be anything that is Sized
/// output type should be your Model type
pub trait ToModel {
    type Args;
    type Output;
    fn to_model(self, args: Self::Args) -> Self::Output;
}