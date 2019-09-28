use clap::{App, Arg};


fn main() {
    let matches = App::new("myprog")
        .arg(Arg::with_name("eff")
            .short("f"))
        .arg(Arg::with_name("pea")
            .short("p")
            .takes_value(true))
        .arg(Arg::with_name("slop")
            .multiple(true)
            .last(true))
        .get_matches();

    println!("-f used: {:?}", matches.is_present("eff"));
    println!("-p's value: {:?}", matches.value_of("pea"));
    println!("'slops' values: {:?}", matches.values_of("slop").map(|vals| vals.collect::<Vec<_>>()));
}
