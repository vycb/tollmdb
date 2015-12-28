extern crate rusttol;
use rusttol::node::{Node};
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
use rustc_serialize::json::{ToJson};
extern crate lmdb_rs as lmdb;
use std::path::Path;
use lmdb::{EnvBuilder, DbFlags, Environment, DbHandle};
use std::env;

impl rusttol::DataStore for LMDBClient {
	
	fn save(&self, node: &Node) {
		
		let parent = node.parent.clone().unwrap();
	   
	    debug!("Save id:{} name:{} p.id:{} p.name:{} oname:{} desc:{}",
	                 &node.id, &node.name, &parent.id, &parent.name, &node.othername, &node.description);
	    
	    let txn = self.env.new_transaction().unwrap();
	    {
	        let db = txn.bind(&self.db);
	
            db.set(&(node.id.clone()+":"+&node.name.clone()), &node.to_json().to_string()).unwrap();
	    }
	
	    match txn.commit() {
	        Err(e) => panic!("LMDBClient failed to commit:{}", e),
	        Ok(_) => ()
	    }
	    
	}
}

struct LMDBClient {
	env: Environment,
	db: DbHandle
}

impl LMDBClient {

	pub fn new() -> LMDBClient {
		let eb = EnvBuilder::new();
		eb.map_size(100001024 as u64);
		let env = eb.open(&Path::new("tol-lmdb"), 0o777).unwrap();
		let db = env.get_default_db(DbFlags::empty()).unwrap();
		
		LMDBClient{
			env: env,
			db : db
		}
	}
	
	pub fn read(&self, key: &str) {
	
	    let reader = self.env.get_reader().unwrap();
	    let db = reader.bind(&self.db);
	    
//	    let name = db.get::<&str>(&"Smith").unwrap();
//	    let mut cursor = db.new_cursor().unwrap();
	    for d in db.item_iter(&key).unwrap() {
	    	println!("key:{} val:{}", d.get_key::<&str>(), d.get_value::<&str>());
	    }
	}
}


fn main(){
	env_logger::init().unwrap();
	
	let lmdbc = LMDBClient::new();
	
	let args: Vec<String> = env::args().collect();
	
	match args.len() {
        3 => if args[1] == "query" { lmdbc.read( &args[2].clone() ) },
		_ => 
			rusttol::xml_walk(&lmdbc)
    } 
	
}




