mod jsonxs;

pub use crate::jsonxs::JsonXsSerializer;
pub use crate::jsonxs::JsonXsValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
