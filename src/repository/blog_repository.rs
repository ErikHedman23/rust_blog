use crate::model::blog_post::Post;
use leptos::*;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    //sql update / insert query here later
    Ok(String::new("placeholder"))
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    //sql query here later
    Ok(Post {
        id: "1".to_string(),
        dt: Local::now().naive_local(),
        title: "Ocean".to_string(),
        image_url: "https://bit.ly/3t0bA61".to_string(),
        text: "I spent some time in the ocean. It was a beautiful day.".to_string(),
    })
}
