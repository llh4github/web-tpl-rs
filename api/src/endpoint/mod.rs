use utoipa_actix_web::service_config::ServiceConfig;

mod demo01;
mod user;

pub(crate) fn apis(c: &mut ServiceConfig) {
    demo01::register_api(c);
    user::register_api(c);
}
