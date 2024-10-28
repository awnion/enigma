pub trait Decoder {
    type Letter;

    fn decode<I>(&self, input: I) -> Self::Letter
    where
        I: Into<Self::Letter>;
}
