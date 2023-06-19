#[cfg(feature = "sled-storage")]
mod hello_world {
    use std::ops::Div;
    use {
        gluesql::{
            prelude::{Glue, Payload, Value},
            sled_storage::SledStorage,
        },
        std::fs,
    };
    const PRICE : usize = 1;

    pub async fn run() {
        /*
            Initiate a connection
        */
        /*
            Open a Sled database, this will create one if one does not yet exist
        */
        let sled_dir = "/tmp/gluesql/tour";
        fs::remove_dir_all(sled_dir).unwrap_or(());
        let storage = SledStorage::new(sled_dir).expect("Something went wrong!");
        /*
            Wrap the Sled database with Glue
        */
        let mut glue = Glue::new(storage);

        /*
            Create table then insert a row

            Write queries as a string
        */
        let queries = "
            CREATE TABLE accommodation (id INTEGER, name TEXT, price INTEGER);
            INSERT INTO accommodation VALUES (1, 'St Johns Hotel', 60000);
            INSERT INTO accommodation VALUES (2, 'Pd Hotel', 50000);
        ";

        glue.execute(queries).await.expect("Execution failed");

        /*
            Select inserted row
        */
        let queries = "
            SELECT name, price FROM accommodation
        ";

        let result = glue.execute(queries).await.expect("Failed to execute");

        /*
            Query results are wrapped into a payload enum, on the basis of the query type
        */
        println!("{:?}",result);
        assert_eq!(result.len(), 1);

        let rows = match &result[0] {
            Payload::Select { labels: _, rows } => rows,
            _ => panic!("Unexpected result: {:?}", result),
        };

        assert_eq!(rows.len(), 2);

        let mut total_price = 0;
        for accommodation in rows.iter() {
            let price = match accommodation[PRICE] { Value::I64(price) => price , _=> 0};
            total_price += price
        }

        assert_eq!(total_price, 110000);
        println!("숙박 평균가 = {}",total_price.div(rows.len() as i64))
    }
}

fn main() {
    #[cfg(feature = "sled-storage")]
    futures::executor::block_on(hello_world::run());
}
