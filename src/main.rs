use actix_web::{
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer,
};
use serde::Deserialize;
use std::io::Result;

#[derive(Deserialize)]
struct GcdParameter {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    server.bind("127.0.0.1:3000")?.run().await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
           <title>GCD Calculator</title>
           <form action="/gcd" method="post">
           <input type="text" name="n"/>
           <input type="text" name="m"/>
           <button type="submit">Compute GCD</button>
           </form>
        "#,
    )
}

async fn post_gcd(form: Form<GcdParameter>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} \
        is <b>{}</b>\n",
        form.n,
        form.m,
        greatest_common_divisor(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;

            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}

#[test]
fn test_gcd() {
    assert_eq!(greatest_common_divisor(14, 15), 1);

    assert_eq!(
        greatest_common_divisor(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19,),
        3 * 11
    );
}
