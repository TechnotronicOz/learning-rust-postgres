    use postgres::{Client, NoTls, Error};
    use std::collections::HashMap;
    use std::env;

    #[derive(Debug, Clone)]
    struct Market {
        _id: i64,
        currency: String,
        maker_id: i64,
    }

    fn get_env_var(n: String) -> String {
        match env::var(n) {
            Ok(val) => val,
            Err(_e) => "none".to_string(),
        }
    }

    fn main() -> Result<(), Error> {
        let db_user = get_env_var(String::from("DB_USER"));
        let db_pass = get_env_var(String::from("DB_PASS"));
        let db_port = get_env_var(String::from("DB_PORT"));
        let db_host = get_env_var(String::from("DB_HOST"));
        let db_name = get_env_var(String::from("DB_NAME"));
        let url = format!("postgresql://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name);

        let mut client = Client::connect(&url,
                                         NoTls).unwrap();

        let mut markets: HashMap<i64, Market> = HashMap::new();
        for row in client.query("SELECT id, maker_id, currency FROM market", &[])? {
            let row_id: i64 = row.get(0);
            markets.insert(row_id, Market {
                _id: row_id,
                maker_id: row.get(1),
                currency: row.get(2),
            });
            // let mref = markets.get(&row_id).unwrap();
            // println!("Market id={} currency={} maker_id={}", mref._id, mref.currency, mref.maker_id);
        }

        println!("markets={:#?}", markets);
        Ok(())

    }
