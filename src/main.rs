extern crate clap;
extern crate jmespath;
extern crate serde;
extern crate serde_json;

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

    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
    println!("{}", json);

    let expr = jmespath::compile("foo.bar").unwrap();

    // Parse some JSON data into a JMESPath variable
    let json_str = r#"{"foo": {"bar": true}}"#;
    let data = jmespath::Variable::from_json(json_str).unwrap();

    // Search the data with the compiled expression
    let result = expr.search(data).unwrap();
    println!("{}", result);
}
