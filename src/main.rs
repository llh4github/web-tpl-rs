use common::cfg::Settings;

fn main() {
    let cfg = Settings::new();
    match cfg {
        Ok(c) => {
            api::main(c);
        }
        Err(e) => {
            println!("读取配置失败 {:?}", e);
            std::process::exit(1);
        }
    }
}
