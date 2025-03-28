use utoipa_actix_web::service_config::ServiceConfig;

mod demo01;
mod user;
mod login;
mod role;

pub(crate) fn apis(c: &mut ServiceConfig) {
    demo01::register_api(c);
    user::register_api(c);
    login::register_api(c);
    role::register_api(c);
}
