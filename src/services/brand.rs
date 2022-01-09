use crate::db;
use crate::models::Brand;
pub async fn find() -> Vec<Brand> {
    if let Ok(coll) = db::collection::<Brand>("brands").await {
        return vec![];
    }

    vec![]
}
