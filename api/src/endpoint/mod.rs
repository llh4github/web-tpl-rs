use utoipa_actix_web::service_config::ServiceConfig;

mod demo01;
mod user;

pub(crate) fn apis(c: &mut ServiceConfig) {
    c.service(demo01::get_student);
    c.service(demo01::add_student);
    c.service(user::find_user);
    c.service(user::add_user);
    c.service(user::page_query);
    c.service(user::update_pwd);
}
