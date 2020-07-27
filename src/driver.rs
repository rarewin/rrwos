pub mod interface {
    pub trait DeviceDriver {
        fn compatible(&self) -> &'static str;
        fn init(&mut self) -> Result<(), ()>;
    }
}
