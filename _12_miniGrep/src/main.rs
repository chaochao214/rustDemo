use std::env;


mod paramerger;

use _12_miniGrep::*;

/**
* 启动命令 cargo  run  a  b
*/

fn main() {

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    run(config);
}


