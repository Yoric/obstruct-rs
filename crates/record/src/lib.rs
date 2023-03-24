pub trait Field<T> {
    const NAME: &'static str;
    fn take(self) -> T;
}
