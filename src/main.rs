#[macro_use]
extern crate serde_json;
extern crate hex;
extern crate urlencoding;

struct UserIn {
    inp: String,
}

struct Jsoninp {
    inp: serde_json::Value,
}

impl UserIn {
    fn get_hex(&self) -> String {
        format!("{}", hex::encode(&self.inp.to_string()))
    }
    fn get_url(&self) -> String {
        format!("{}", urlencoding::encode(&self.inp.to_string()))
    }
}

impl Jsoninp {
    fn beautify(&self) {
        println!("{}", serde_json::to_string_pretty(&self.inp).unwrap());
    }
}



fn main() {
    let mut lst = vec![];
    let sysargs = std::env::args();
    let mut iter: i32 = 0;
    let mut user_input = UserIn {
        inp: "Hello".to_string(),
    };

    // Put User Inputs into List "lst"
    for x in sysargs {
        if iter != 0 {
            user_input.inp = x.to_string();
            lst.push(vec![
                iter.to_string(),
                x,
                user_input.get_hex().to_string(),
                user_input.get_url().to_string(),
            ]);
            iter += 1;
        } else {
            iter += 1;
        }
    }

    // Beautify the Json and Output
    //let mut obj = json!({"Temp":1});
    for item in lst {
        let json_inp = Jsoninp {inp:json!({format!("Arg {}",item[0]):{"Value":item[1], "Hex":item[2], "Url":item[3]}})};
        json_inp.beautify();
    }
}







//------
