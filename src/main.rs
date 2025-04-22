use std::{env, net::IpAddr,str::FromStr};



struct Arguments{
    thread: u16,
    ip_addr: IpAddr,
    flag: String
}

impl Arguments{
    fn new(args: &[String])->Result<Arguments,&'static str>{
        if args.len()<2{
            return &Err("not enough arguments");
        }else if args.len()>4{
            return  &Err("too many arguments");
        }

        let f = args[1].clone(); 

        if let Ok(ip_addr) = IpAddr::from_str(&f){
            return Ok(Arguments { thread: 4, ip_addr: ip_addr, flag: String::from("")});
        }else{
            let flag =args[1].clone();
            if flag.contains("-h") || flag.contains("--help")&&args.len()==2{
                println!("-j for selecting how many threads for operation");
            }
            return Err("help");
        }
    }
}
fn main() {

    let args:Vec<String> = env::args().collect();

    for i in args{
        println!("{}",i);
    }
}
