use actix_web::{get, HttpResponse, Responder};

use sailfish::TemplateOnce;

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
    pub fn run(&self) -> Result<String, &'static str> {
        let index = self.clone().render_once().unwrap();
        Ok(index)
    }
}

#[get("/")]
pub async fn login() -> impl Responder {
    let body = IndexPage::default().run().unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
