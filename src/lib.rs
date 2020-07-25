mod jsonxs;

pub use jsonxs::JsonXsSerializer;
pub use jsonxs::JsonXsValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
