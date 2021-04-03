use log::{debug, info};
use sailfish::TemplateOnce;
use tokio::fs;
use tokio::io::{Error, ErrorKind};

#[derive(TemplateOnce)] // automatically implement `TemplateOnce` trait
#[template(path = "index.stpl")] // specify the path to template
struct IndexPage {
    // data to be passed to the template
    name: String,
    title: String,
}

const BASE_DIR: &str = "./output";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    match fs::create_dir(BASE_DIR).await {
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                info!("cleaning up old assetes");
                fs::remove_dir_all(BASE_DIR).await.unwrap();
                debug!("creating target location");
                fs::create_dir(BASE_DIR).await.unwrap();
            }
        }
        _ => (),
    };

    let ctx = IndexPage {
        name: "mCaptcha".into(),
        title: "Login".into(),
    };

    // Now render templates with given data
    info!("rendering {}", path("index.html"));
    let index = ctx.render_once().unwrap();
    fs::write(path("index.html"), index).await.unwrap();
    info!("wrote {}", path("index.html"));

    let ctx = signup::IndexPage {
        name: "mCaptcha".into(),
        title: "Register".into(),
    };

    // Now render templates with given data
    info!("rendering {}", path("signup/index.html"));
    let index = ctx.render_once().unwrap();
    fs::create_dir(path("signup")).await.unwrap();
    info!("creating dir {}", path("signup/"));

    fs::write(path("signup/index.html"), index).await.unwrap();
    info!("wrote {}", path("signup/index.html"));
}

fn path(rel: &str) -> String {
    format!("{}/{}", BASE_DIR, rel)
}

mod signup {
    use super::*;
    #[derive(TemplateOnce)] // automatically implement `TemplateOnce` trait
    #[template(path = "signup/index.stpl", escape = false)] // specify the path to template
    pub struct IndexPage {
        // data to be passed to the template
        pub name: String,
        pub title: String,
    }
}
