use rocket::{fairing::Fairing, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub fn get_cors_fairing() -> impl Fairing {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    cors.to_cors().unwrap()
}
