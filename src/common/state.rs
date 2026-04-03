use std::any::Any;

pub trait State: Any {
    fn as_any(&self) -> &dyn Any;
}
