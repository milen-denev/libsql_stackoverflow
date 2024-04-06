use libsql::{Builder, Connection};


#[tokio::main]
async fn main() {
    let builder = Builder::new_remote("libsql://<subdomain>.turso.io".into(), "<token>".into()).build().await.unwrap();
    let connection = builder.connect().unwrap();
    get_current_db_schema("Test", &connection).await;
}

pub async fn get_current_db_schema(name: &str, connection: &Connection) -> Option<Vec<String>> {  
    let query = format!("SELECT sql FROM sqlite_schema WHERE name = {}", name);

    let result_query = connection
         .query(&query, ())
         .await;

    if let Ok(mut result_row) = result_query {
         let result_rows = result_row
              .next()
              .await;

         if let Ok(_result_row) = result_rows {
              return None;
         }
         else {
              return None;
         }

    }
    else {
         return None;
    }
}
