pub trait Widget: Send + Sync {
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}