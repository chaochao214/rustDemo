#[cfg(test)]
mod tests {
use std::env;
    #[test]
    fn test_print() {
        println!("________________demo");
    }


    #[test]
    fn test_env_args() {
        // cargo run needle haystack
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
    }

    /**
     *@Description
     *@Date 2021/12/25 14:40
    **/
    #[test]
    fn test_parameter() {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);

        let query = &args[1];
        let filename = &args[2];

        println!("----------");
        println!("Searching for {}", query);
        println!("In file {}", filename);
        println!("{:?}", args);

    }
    /**
     *@Description
     *@Date 2021/12/25 14:38
    **/ 
    #[test]
    fn test_demo() {

    }
}

