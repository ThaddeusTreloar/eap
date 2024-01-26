use crate::environment::Environment;

pub trait Config {
    fn parse<T: Environment>(backend: T) -> Self;
    
    fn parse_env<T: Environment+Default>() -> Self
    where Self: Sized
    {
        Self::parse(T::default())
    }
}