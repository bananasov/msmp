pub fn generate_types<'a>(data: &'a str) {

}

fn split_methods_and_notifs() {

}
// TODO: holy shit im tired what also maybe make the types singular instead of plural, aka method.rs instead of methods.rs
#[cfg(test)]
mod tests {
    use crate::generate_types;

    static RPC_DISCOVER_RESPONSE: &'static str = include_str!("../../../data/rpc.discover-response.json");

    #[test]
    fn test_generate_types() {
        generate_types(RPC_DISCOVER_RESPONSE);
    }
}