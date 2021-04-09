use cache_buster::Files;
use lazy_static::lazy_static;
use log::{debug, info};
use sailfish::TemplateOnce;
use tokio::fs;
use tokio::io::{Error, ErrorKind};

#[derive(Clone, TemplateOnce)]
#[template(path = "auth/login/index.html")]
struct IndexPage {
    name: String,
    title: String,
}

impl Default for IndexPage {
    fn default() -> Self {
        IndexPage {
            name: "mCaptcha".into(),
            title: "Login".into(),
        }
    }
}

impl IndexPage {
    pub async fn run(&self) -> Result<(), Error> {
        let file = root_path("index.html");

        info!("rendering {}", &file);
        let index = self.clone().render_once().unwrap();

        fs::write(&file, index).await?;
        info!("wrote {}", &file);
        Ok(())
    }
}

const BASE_DIR: &str = "./prod";

lazy_static! {
    pub static ref FILES: Files = Files::load();
}

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

    IndexPage::default().run().await.unwrap();
    signup::IndexPage::default().run().await.unwrap();
    panel::IndexPage::default().run().await.unwrap();
}

fn root_path(rel: &str) -> String {
    format!("{}/{}", BASE_DIR, rel)
}

fn rel_path(dir: &str, file: &str) -> String {
    format!("{}/{}", dir, file)
}

mod signup {
    use super::*;

    #[derive(TemplateOnce, Clone)]
    #[template(path = "auth/register/index.html")]
    pub struct IndexPage {
        pub name: String,
        pub title: String,
    }

    impl Default for IndexPage {
        fn default() -> Self {
            IndexPage {
                name: "mCaptcha".into(),
                title: "Join".into(),
            }
        }
    }

    impl IndexPage {
        pub async fn run(&self) -> Result<(), Error> {
            let dir = root_path("register");
            let file = rel_path(&dir, "index.html");

            print!("");
            info!("rendering {}", &file);
            let index = self.clone().render_once().unwrap();

            fs::create_dir(&dir).await?;
            info!("creating dir {}", &dir);

            fs::write(&file, index).await?;
            info!("wrote {}", &file);
            Ok(())
        }
    }
}

pub type Literal = &'static str;

pub mod panel {
    use super::*;
    use section::*;

    #[derive(TemplateOnce, Clone)]
    #[template(path = "panel/index.html")]
    pub struct IndexPage {
        pub name: String,
        pub title: String,
        pub active: &'static SubPanel,
    }

    const TITLE: &str = "Dashboard";

    impl Default for IndexPage {
        fn default() -> Self {
            IndexPage {
                name: "mCaptcha".into(),
                title: "Home".into(),
                active: &COMMENTS,
            }
        }
    }

    impl IndexPage {
        pub async fn run(&self) -> Result<(), Error> {
            let dir = root_path("panel");
            let file = rel_path(&dir, "index.html");

            info!("rendering {}", &file);
            let index = self.clone().render_once().unwrap();

            fs::create_dir(&dir).await?;
            info!("creating dir {}", &dir);

            fs::write(&file, index).await?;
            info!("wrote {}", &file);
            Ok(())
        }
    }

    pub mod section {
        use super::*;

        pub struct Section<const N: usize> {
            pub name: Literal,
            pub elements: [&'static SubPanel; N],
        }

        pub struct SubPanel {
            pub name: Literal,
            pub icon: Literal,
        }

        macro_rules! sub_panel {
            ($var:ident, $name:expr, $icon:expr) => {
                pub static $var: SubPanel = SubPanel {
                    name: $name,
                    icon: $icon,
                };
            };
        }

        sub_panel!(COMMENTS, "Comments", "comments");
        sub_panel!(USERS, "Users", "users");
        sub_panel!(PAGES, "Pages", "pages");

        pub static ADMIN_SECTION: Section<3> = Section {
            elements: [&COMMENTS, &USERS, &PAGES],
            name: "Admin",
        };

        pub static SETTINGS_SECTION: Section<3> = Section {
            elements: [&COMMENTS, &USERS, &PAGES],
            name: "Settings",
        };
    }
}
