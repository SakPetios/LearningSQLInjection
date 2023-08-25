#[macro_use]
extern crate rocket;
use r2d2_mysql::{
    mysql::{prelude::*, Opts, OptsBuilder},
    r2d2::{self, Pool},
    MySqlConnectionManager,
};
use rocket::{
    form::{Form, FromForm},
    http::Status,
    State,
};
use std::{env, sync::Arc};
extern crate dotenv;
use rocket_dyn_templates::Template;
#[derive(FromForm, Debug)]
struct LoginParams {
    username: String,
    password: String,
}

#[post("/login-post", data = "<params>")]
async fn login_post(
    params: Form<LoginParams>,
    pool: &State<Arc<Pool<MySqlConnectionManager>>>,
) -> Result<Template, Status> {
    // println!("{:?}",params);
    let mut conn = pool.get().unwrap();
    let data = conn
        .query_first::<String, &str>(&format!("SELECT Password FROM UserCredentials WHERE Username='{}'",params.username))
        .unwrap();
    if let Some(password) = data {
        if password == params.password { 
            return Ok(Template::render("welcome", {}))
        }
    }
    Err(Status::Forbidden)
}

#[get("/login")]
async fn login() -> Template {
    Template::render("index", {})
}
#[get("/login-get?<params..>")]
async fn login_get(params: LoginParams,pool: &State<Arc<Pool<MySqlConnectionManager>>>) -> Result<Template, Status> {
    let mut conn = pool.get().unwrap();
    let data = conn
        .query_first::<String, &str>(&format!("SELECT Password FROM UserCredentials WHERE Username='{}'",params.username))
        .unwrap();
    if let Some(password) = data {
        if password == params.password { 
            return Ok(Template::render("welcome", {}))
        }
    }
    Ok(Template::render("welcome", {}))
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let url = env::var("DB_URL").expect("No url found :(");
    let opts = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);
    let pool = Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap());
    rocket::build()
        .mount("/", routes![login_post, login, login_get])
        .manage(pool)
        .attach(Template::fairing())
}
