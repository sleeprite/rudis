#[cfg(test)]
mod test_performance {

    use redis::{Client, Commands, Connection};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().unwrap()
    }

    #[test]
    fn test_set() {
        let mut con = setup();
        for i in 0..100000 {
            let _: () = con.set(i.to_string(), i.to_string()).unwrap();
        }
    }

    #[test]
    fn test_get() {
        let mut con = setup();
        for _i in 0..100000 {
            let _: () = con.get("user").unwrap();
        }
    }
}