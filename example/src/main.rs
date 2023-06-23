use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    pub id: Thing,
    pub stuff: String,
}

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    let mut msg = String::from("");
    for _ in 1..16 << 20 {
        msg.push('a');
    }

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("namespace").use_db("database").await.unwrap();

    let _content: Content = db
        .update(("table", "testID"))
        .content(Content {
            id: Thing {
                tb: "table".to_owned(),
                id: surrealdb::sql::Id::String("testID".to_owned()),
            },
            stuff: msg.to_owned(),
        })
        .await?;

    dbg!(_content);

    Ok(())
}
