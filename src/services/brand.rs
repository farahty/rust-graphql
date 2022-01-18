use crate::db;
use crate::models::Brand;
use futures::TryStreamExt;

pub async fn find() -> Vec<Brand> {
    match db::collection::<Brand>("brands").await {
        Ok(coll) => {
            let mut brands: Vec<Brand> = vec![];

            let mut cursor = coll.find(None, None).await.expect("failed to load brands");

            while let Some(brand) = cursor.try_next().await.unwrap() {
                println!("{:?}", brand);
                brands.push(brand);
            }

            brands
        }
        Err(_) => vec![],
    }
}
