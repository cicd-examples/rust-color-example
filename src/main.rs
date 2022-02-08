extern crate log;
extern crate env_logger;

use std::env;
//use log::{debug, error, log_enabled, info, Level};
use phf::phf_map;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::Serialize;

static HTML_PAGE: &str = "<!DOCTYPE html>
<html>
<head>
<script type=\"text/javascript\">
function setColor(data) {
    console.log(data);
    document.querySelector(\"#color_text\").innerText = `Color: ${data.color}`;
    document.querySelector(\"#color_box\").style.backgroundColor = data.color;
}
function getColor() {
    fetch('/api/color')
      .then(response => response.json())
      .then(data => setColor(data));
}
document.addEventListener(\"DOMContentLoaded\", getColor);
</script>
</head>
<body style=\"min-height: 100%;\">
  <h1>Test Color Page</h1>
  <h2 id=\"color_text\">Color: #000000</h2>
  <div id=\"color_box\" style=\"backgroundColor: #000000; height: 300px; width: 300px\">
    <br />
  </div>
</body>
</html>
";

static COLOR_TABLE : phf::Map<&'static str, &'static str> = phf_map! {
    "red" => "#CC0000",
    "blue" => "#0000CC",
    "green" => "#00CC00",
    "yellow" => "#CCCC00",
    "purple" => "#CC66CC",
    "grey" => "#CCCCCC",
    "orange" => "#FFCC99"
};

struct EnvColor {
    color: String
}

#[derive(Serialize)]
struct Color {
    color: String
}

fn check_color() -> Result<String, String> {
    let tmp_color = env::var("COLOR").expect("Color not set!");
    if (tmp_color.chars().count() == 7) && (tmp_color.chars().next().unwrap() == '#') {
        Ok(tmp_color)
    } else if COLOR_TABLE.contains_key(&tmp_color.to_lowercase()) {
        Ok(String::from(COLOR_TABLE.get(&tmp_color.to_lowercase()).unwrap().clone()))
    } else {
        // FIXME: Figure out how to print keys of the hashmap.
        Err(String::from("Invalid color set.  Please use one of [the specified colors] or a six digit hex-code."))
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(HTML_PAGE)
}

#[get("/api/color")]
async fn get_color(data: web::Data<EnvColor>) -> impl Responder {
    let tmp_color = Color {
        color: String::from(&data.color)
    };
    web::Json(tmp_color)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(EnvColor {
                color: check_color().expect("Invalid Color")
            })
            .service(index)
            .service(get_color)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}