use actix_web::{get, HttpResponse, Responder};
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone)]
#[template(path = "panel/index.html")]
pub struct IndexPage {
    pub name: String,
    pub title: String,
}

const TITLE: &str = "Dashboard";

impl Default for IndexPage {
    fn default() -> Self {
        IndexPage {
            name: "mCaptcha".into(),
            title: "Home".into(),
        }
    }
}

impl IndexPage {
    pub fn run(&self) -> Result<String, &'static str> {
        let index = self.clone().render_once().unwrap();
        Ok(index)
    }
}

#[get("/panel")]
pub async fn panel() -> impl Responder {
    let body = IndexPage::default().run().unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
