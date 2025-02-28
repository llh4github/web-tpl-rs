use utoipa_actix_web::service_config::ServiceConfig;

mod demo01;
pub(crate) fn registers(c: &mut ServiceConfig) {
    c.service(demo01::get_student);
    c.service(demo01::add_student);
}
