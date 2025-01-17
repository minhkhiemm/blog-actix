use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let app = blog_actix::Blog::new(8998);
    let result = app.run(database_url);
    println!("{:?}", result);
}
