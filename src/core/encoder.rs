pub trait Encoder {
    type Letter;

    fn encode(&self, input: Self::Letter) -> Self::Letter;
}
