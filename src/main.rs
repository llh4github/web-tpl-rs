use common::cfg::AppCfgs;

fn main() {
    let cfg = AppCfgs::new();
    match cfg {
        Ok(c) => {
            api::main(&c);
        }
        Err(e) => {
            println!("读取配置失败 {:?}", e);
            std::process::exit(1);
        }
    }
}
