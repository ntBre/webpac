use actix_files::NamedFile;
use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use psqs::program::mopac::Mopac;
use rust_pbqff::{config::Config, coord_type::normal::Normal};
use std::{error::Error, fmt::Write};

macro_rules! input {
    ($w:expr, $name:expr, $label:expr, $value:expr) => {
        write!(
            $w,
            r#"
<br>
<label for="{}">{}</label>
<input type="number" id="{}" name="{}", value="{}">
"#,
            $name, $label, $name, $name, $value
        )
        .unwrap();
    };
}

async fn index() -> impl Responder {
    let content = std::fs::read_to_string("static/index.html").unwrap();

    let mut body = String::new();
    input!(body, "charge", "Charge", 0);
    input!(body, "step_size", "Step size", 0.005);
    input!(body, "sleep_int", "Sleep interval", 10);
    input!(body, "job_limit", "Max jobs", 1024);
    input!(body, "chunk_size", "Chunk size", 1);
    input!(body, "check_int", "Checkpoint interval (0 to disable)", 100);

    let content = content.replace("{{.body}}", &body);
    HttpResponse::Ok().body(content)
}

async fn run(
    item: String,
) -> std::result::Result<HttpResponse, Box<dyn Error>> {
    let config: Config = serde_json::from_str(&item)?;
    use rust_pbqff::coord_type::CoordType;
    let (s, o) = <rust_pbqff::coord_type::normal::Normal as CoordType<
        _,
        _,
        Mopac,
    >>::run(
        Normal::findiff(false),
        &mut std::io::stderr(),
        &psqs::queue::local::Local::new(
            "/tmp",
            config.chunk_size,
            "/opt/mopac/mopac",
        ),
        &config,
    );
    let body = format!("{s}\n{o}");
    Ok(HttpResponse::Ok().body(body))
}

macro_rules! file_handlers {
    ($($name:ident => $path:expr$(,)*)*) => {
	$(
	    async fn $name(req: HttpRequest) -> Result<NamedFile> {
		let dir = std::path::Path::new($path);
		let path: std::path::PathBuf = req.match_info()
		    .query("filename").parse().unwrap();
		Ok(NamedFile::open(dir.join(path))?)
	    }
	)*
    }
}

file_handlers! {
    js_file => "js/",
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/run").route(web::post().to(run)))
            .route("/js/{filename:.*}", web::get().to(js_file))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
