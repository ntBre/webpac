use actix_files::NamedFile;
use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use rust_pbqff::config::{Config, CoordType, Geom, Program, Queue};
use std::{error::Error, fmt::Write, sync::RwLock};

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
    data: web::Data<State>,
    item: String,
) -> std::result::Result<HttpResponse, Box<dyn Error>> {
    let config: Config = serde_json::from_str(&item)?;
    let mut d = data.config.write().unwrap();
    *d = config;
    Ok(HttpResponse::Ok().finish())
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

struct State {
    config: RwLock<Config>,
}

impl State {
    fn new() -> Self {
        let config = Config {
            geometry: Geom::Zmat(String::new()),
            optimize: false,
            charge: 0,
            step_size: 0.005,
            coord_type: CoordType::Normal,
            template: String::new(),
            program: Program::Mopac,
            queue: Queue::Local,
            sleep_int: 10,
            job_limit: 1024,
            chunk_size: 1,
            findiff: false,
            check_int: 100,
        };
        Self {
            config: RwLock::new(config),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State::new());
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::resource("/run").route(web::post().to(run)))
            .route("/js/{filename:.*}", web::get().to(js_file))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
