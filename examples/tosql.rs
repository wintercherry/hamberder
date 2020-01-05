/*
   run with cargo run --example tosql --release
   converts a json file with 100k (they repeat every 1000 rows because it's
   just a stupid example) rows called input.json with the form
   [row1,row2,...]
   where each row looks like
   {"id":1,"first_name":"Audy","last_name":"Taborre","lat":-17.3058881,"long":31.5655424},
   into equivalent INSERT statements

   note: there is zero error-handling in here for the sake of brevity

   the tags that come back from the parser in this case look like
   BeginArray,
       BeginObject,
           ObjectKey, Number,
           ObjectKey, StringLiteral,
           ObjectKey, StringLiteral,
           ObjectKey, Number,
           ObjectKey, Number,
       EndObject
   EndArray
*/

macro_rules! force_unwrap {
    ($enum:path, $expr:expr) => {{
        let res = $expr.unwrap(); // only eval once
        if let $enum(item) = res {
            item
        } else {
            eprintln!("Encountered unexpected value: {:?}", res);
            panic!()
        }
    }};
}

fn main() {
    use hamberder::parser::Tag;
    let tags = hamberder::parse_file("examples/input.json").unwrap();
    //let v : hamberder::parser::TagVec = tags.iter().collect();
    //println!("dumping tags: {:?}", v);
    tags.recv().unwrap(); // skip BeginArray
    loop {
        // the following tag is either a new BeginObject or the EndArray signifying
        // we're done
        match tags.recv().unwrap()
         {
             Tag::BeginObject => {
                tags.recv().unwrap(); // "id"
                let id = force_unwrap!(Tag::Number, tags.recv());
                tags.recv().unwrap(); // "first_name"
                let first_name = force_unwrap!(Tag::StringLiteral, tags.recv());
                tags.recv().unwrap(); // "last_name"
                let last_name = force_unwrap!(Tag::StringLiteral, tags.recv());
                tags.recv().unwrap(); // "latitude"
                let latitude = force_unwrap!(Tag::Number, tags.recv());
                tags.recv().unwrap(); // "longitude"
                let longitude = force_unwrap!(Tag::Number, tags.recv());
                println!(
                    "INSERT INTO \"my_table\" VALUES ({}, \"{}\", \"{}\", {}, {});",
                    id, first_name, last_name, latitude, longitude
                );
                tags.recv().unwrap(); // end object
             },
             Tag::EndArray => break,
             _ => panic!("unexpected")
        }
    }
}
