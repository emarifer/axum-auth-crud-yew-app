#[derive(Debug, Clone)]
pub struct Config {
    pub supabase_url: String,
    pub supabase_anon_key: String,

    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let supabase_url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let supabase_anon_key =
            std::env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");

        let jwt_secret =
            std::env::var("JWT_SECRET").unwrap_or("my_ultra_secure_secret".to_string());
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").unwrap_or("60m".to_string());
        let jwt_maxage = std::env::var("JWT_MAXAGE").unwrap_or("60".to_string());

        Config {
            supabase_url,
            supabase_anon_key,

            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}
