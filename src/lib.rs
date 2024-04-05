use std::ops::{Deref, Index};

pub struct Flarr<T>(Vec<T>);

impl<T> Flarr<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T> Index<f64> for Flarr<T> {
    type Output = T;

    fn index(&self, index: f64) -> &Self::Output {
        assert!((index >= 0.) && (index < 1.));

        // println!("{}", (self.0.len() as f64 * index).round() as usize);

        &self.0[(self.0.len() as f64 * index).round() as usize]
    }
}

// impl<T> Deref for Flarr<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         Self.0
//     }
// }
