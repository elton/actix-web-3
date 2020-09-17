use actix_web::web;

mod extractors;
mod json;

pub fn routes(cfg: &mut web::ServiceConfig) {
    // curl https://localhost:8443/handlers/str
    cfg.service(json::responder_str);
    // curl https://localhost:8443/handlers/string
    cfg.service(json::responder_string);
    // curl https://localhost:8443/handlers/impl_responder
    cfg.service(json::responder_impl_responder);
    // curl https://localhost:8443/handlers/custom_responder
    cfg.service(json::responder_custom_responder);
    // curl -i -H 'Content-Type: application/json' -d '{"name": "Test user", "number": 100}' -X POST https://localhost:8443/handlers/extractor
    cfg.service(json::extractor);
    // curl -i -H 'Content-Type: application/json' -d '{"name": "Test user", "number": 100}' -X POST https://localhost:8443/handlers/extractor2
    cfg.service(json::extract_item);
    // curl https://localhost:8443/handlers/users/10/elton
    cfg.service(extractors::user_info);
    // curl https://localhost:8443/handlers/users2/10/elton
    cfg.service(extractors::user_info_serde);
    // curl https://localhost:8443/handlers/users3/10/elton
    cfg.service(extractors::user_info_request);
    // curl -i -H 'Content-Type: application/x-www-form-urlencoded' -d 'username="elton zheng"' -X POST https://localhost:8443/handlers/user
    // 提交表单
    cfg.service(extractors::form);
}
