use std::process::id;

/*
 * 启动服务
 */
pub fn println_banner(port: String) {
    let version = env!("CARGO_PKG_VERSION");
    let pid = id();
    let pattern = format!(
        r#"
         /\_____/\
        /  o   o  \          Rudis {}
       ( ==  ^  == )
        )         (          Bind: {} PID: {}
       (           )
      ( (  )   (  ) )
     (__(__)___(__)__)
    "#, version, port, pid);
    println!("{}", pattern);
}