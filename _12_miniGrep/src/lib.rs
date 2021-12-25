/**
 *@Description
 *@Date 2021/12/25 20:35
**/
#[cfg(test)]
mod tests{
    use super::*;

    /**
     *@Description
     *@Date 2021/12/25 20:37
    **/
    #[test]
    #[allow(non_snake_case)]
    fn test_demo() {
        let query = "duct";
        let contents = "duct";
        assert_eq!(query,contents)
    }



}

#[test]
fn test_demo() {

}







fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}



pub fn run(config: Config) {
    println!("filename -> {}", config.filename);
    println!("query -> {}", config.query);

}

pub struct Config {
    query: String,
    filename: String,
}

impl  Config {
   pub fn new(args:&[String]) ->Config{
        let query = args[1].clone();
        let filename = args[2].clone();
        Config{query,filename}
    }


}