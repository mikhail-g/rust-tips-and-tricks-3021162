#[derive(Debug)]
struct Invitation {
    invitee: String,
    attending: bool,
    message: Option<String>,
}

impl Invitation {
    fn new(invitee: String, attending: bool, message: Option<String>) -> Invitation {
        Invitation {
            invitee,
            attending,
            message,
        }
    }
}

fn main() {
    let inv1 = Invitation::new("Greg".to_string(), true, None);
    let inv2 = Invitation::new("Yoi".to_string(), false, Some("Busy".to_string()));
    dbg!(&inv1);
    dbg!(&inv2);

    println!("{:#?}", inv1);
    println!("{:#?}", inv2);
}
