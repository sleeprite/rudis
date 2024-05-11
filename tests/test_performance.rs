#[cfg(test)]
mod test_performance {

    use redis::{Client, Commands, Connection};

    fn setup() -> Connection {
        let client = Client::open("redis://127.0.0.1:6379/").unwrap();
        client.get_connection().unwrap()
    }

    #[test]
    fn main() {
        
        let mut con = setup();

        for i in 0..100000 {

            let _: () = con.set(i.to_string(), i.to_string()).unwrap();
        }
    
    }
}