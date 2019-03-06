use crate::matrix::Matrix4x4;

pub trait Transform<T> {
    fn transform(&mut self, transformation: &Matrix4x4<T>);
}
