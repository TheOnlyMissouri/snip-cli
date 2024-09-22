
pub trait UseCase {
    type Input;
    type Output;
    type Error;

    fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
