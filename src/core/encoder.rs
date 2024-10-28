pub trait Encoder {
    type Letter;

    fn encode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>;
}
