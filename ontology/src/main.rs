extern crate serde;
extern crate serde_json;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};


use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::str::FromStr;


use serde_json::{Value, Error};

#[macro_use]
extern crate serde_derive;

mod basicElements;
mod ontology;
use ontology::{Offers,Offer,comparing};

mod repository;
use repository::Repository;

fn main() {
    println!("Initializing local offer list and repository");

    let mut offers:Offers = Offers::from_json_file("offerlist.json");
    let repo:Repository = Repository::from_json_file("repository.json");

    Offers::to_msgpack_file("offerlist_msgpack.json", &offers);
    Repository::to_msgpack_file("repository_msgpack.json", &repo);


    println!("Offers: {:?}", offers);

    println!("Compare two elements of the offer list with each other to test the compare function:");
    {
        let a:&Offer = offers.get_offers().get(0).unwrap();
        let b:&Offer = offers.get_offers().get(1).unwrap();

        match comparing(a,b) {
            Ok(_) => println!("If 'a' would be a request, 'b' would be a sufficient offer."),
            Err(e) => println!("There was an error: {:?}", e),
        }

    }
    offers.compress(&repo);
    println!("Compressed offers: {:?}", offers);


    offers.decompress(&repo);
    println!("Decompressed offers!!: {:?}", offers);

}
