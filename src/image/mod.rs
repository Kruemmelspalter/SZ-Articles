use rocket::{delete, fs::TempFile, get, post, put, routes, Route};

#[get("/<id>")]
async fn get_image_meta(id: i64) {
    todo!()
}

#[get("/file/<id>")]
async fn get_image_file(id: i64) {
    todo!() // download file name hint
}

#[get("/file/<filename>", rank = 2)]
async fn get_image_file_by_str(filename: &str) {
    todo!()
}

#[post("/", format = "image/*", data = "<file>")]
async fn post_image(file: TempFile<'_>) {
    todo!()
}

#[put("/<id>", format = "image/*", data = "<file>")]
async fn replace_image(id: i64, file: TempFile<'_>) {
    todo!()
}

#[delete("/<id>")]
async fn delete_image(id: i64) {
    todo!()
}

pub fn get_routes() -> Vec<Route> {
    routes![
        get_image_meta,
        get_image_file,
        get_image_file_by_str,
        post_image,
        replace_image,
        delete_image
    ]
}
