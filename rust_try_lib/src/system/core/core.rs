///This is interface for application and concrete implementation
///Concrete type of this interface should implement inner logic depends on specific crate
pub trait ApplicationCore {
    fn init(&self) {}

    ///provides executation of assembled logics
    fn run(self);
}
