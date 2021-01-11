#[warn(dead_code)]
#[derive(Debug)]

struct Visa {
    number: u8,
    verification_code: u8,
    name: String,
}

struct MasterCard {
    number: u8,
    verification_code: u8,
    name: String,
}

struct BitCredit {
    wallet: u64,
    name: String
}

struct WesternUnion {
    number: u32,
    verification_code: u8,
    name: String,
}

trait CreditCharge {
    fn charge_with_id (&self, id: u32) -> bool;
}

impl CreditCharge for BitCredit {
    fn charge_with_id (&self, id: u32) -> bool {
        id % 2 == self.wallet as u32 % 2
    }
}

fn transact<P: CreditCharge>(card: P) {
    let id = 333;

    if card.charge_with_id(id) {
        println!("Done")
    } else {
        println!("Invalid code")
    }
}

fn main() {
    let card = BitCredit { wallet: 014565413841, name: String::from("Douglas Alves") };
    //println!("{:#?}", card.wallet);
    transact(card);

}
