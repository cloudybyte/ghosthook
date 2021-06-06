use actix_web::{App, web, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use serde::Deserialize;
use std::fs::{read_to_string};
use crate::json::PostPublished;
use serenity::http::Http;
use serenity::model::channel::Embed;
use actix_web::web::JsonConfig;

mod json;

#[actix_web::main]
async fn main() {
    let config = Config::load("config.toml");
    let conf = config.clone();
    let json_config = JsonConfig::default().limit(262144);

    let server = HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(json_config.clone())
            .wrap(Logger::default())
            .route("/post/published", web::post().to(post_published))
    });
    println!("Starting ghosthook on {}", &conf.host_uri);
    server
        .bind(&conf.host_uri)
        .expect(&format!("couldn't bind to address {}", &conf.host_uri))
        .run()
        .await
        .expect("couldn't run server");
}


async fn post_published(json: web::Json<PostPublished>, config: web::Data<Config>) -> HttpResponse {
    let http = Http::default();
    let embed = Embed::fake(|e| {
        e.title(format!("Post published: {}", json.post.current.title.clone()));
        let desc = {
            let mut desc = json.post.current.excerpt.clone();
            if desc.len() > config.preview_length as usize {
                desc = desc[..config.preview_length].to_string();
            }
            desc
        };
        e.description(desc);
        e.url(json.post.current.url.clone());
        if json.post.current.featureImage.is_some() && (json.post.current.featureImage != Some(Box::new(String::from("")))){
            e.image(json.post.current.featureImage.clone().unwrap());
        }
        if json.post.current.primaryAuthor.profileImage != None {
            e.thumbnail(format!("https:{}", json.post.current.primaryAuthor.profileImage.clone().unwrap()));
        }
        let mut authors = String::new();
        let author_count = json.post.current.authors.len();
        let mut i = 1;
        for author in json.post.current.authors.clone() {
            authors.push_str(&format!("{}", author.name));
            if i != author_count {
                authors.push_str(", ");
            }
            i += 1;
        }

        e.author(|x| {
            x.name(authors.clone());
            x
        });
        e.footer(|x| {
            x.text(format!("{} • {}min", authors, json.post.current.readingTime.clone()));
            x
        });
        e.timestamp(chrono::Utc::now().to_rfc3339());
        e
    });

    for wh in &config.discord_webhooks {
        let webhook = http.get_webhook_with_token(wh.id, &wh.token).await.expect("failed to grab webhook");
        webhook.execute(&http, false, |w| {
            w.username(config.webhook_username.clone());
            w.avatar_url(config.webhook_avatar_url.clone());
            w.embeds(vec![embed.clone()]);
            w
        })
            .await;
    }
    HttpResponse::Ok().finish()
}

#[derive(Clone, Deserialize)]
struct Config {
    host_uri: String,
    preview_length: usize,
    webhook_username: String,
    webhook_avatar_url: String,
    discord_webhooks: Vec<DiscordWebhook>,
}

#[derive(Clone, Deserialize)]
struct DiscordWebhook {
    id: u64,
    token: String,
}

impl Config {
    fn load(path: &str) -> Self {
        let config_string = read_to_string(path).expect("couldn't load config from provided path");
        let config: Config =
            toml::from_str(&config_string).expect("couldn't deserialize config");
        config
    }
}

