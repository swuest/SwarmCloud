/**
Contains the structures defining the repository,
as well as functions to load a repository from JSOn or MsgPack files.
*/

extern crate serde;
extern crate serde_json;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};


use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::str::FromStr;
use std;

use serde_json::{Value, Error};

use basicElements::{BasicElement,IntElement,FloatElement,StringElement,MagnitudeElement};
use basicElements::{DiskType,InstructionSet,MemoryGeneration};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperatingSystemRepo {
    system_type: Vec<StringElement>,
}

impl OperatingSystemRepo {
    
    pub fn get_system_type(&self) -> &Vec<StringElement> { &self.system_type }
    pub fn get_system_type_mut(&mut self) -> &mut Vec<StringElement> { &mut self.system_type }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VirtualizationRepo {
    virtualization_type: Vec<StringElement>,
}

impl VirtualizationRepo {

    pub fn get_virtualization_type(&self) -> &Vec<StringElement> { &self.virtualization_type }
    pub fn get_virtualization_type_mut(&mut self) -> &mut Vec<StringElement> { &mut self.virtualization_type}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CPURepo {
    manufacturer: Vec<StringElement>,
    frequency: Vec<FloatElement>,
    instruction_set: Vec<BasicElement<InstructionSet>>
}

impl CPURepo {
    
    pub fn get_manufacturer(&self) -> &Vec<StringElement> { &self.manufacturer }
    pub fn get_manufacturer_mut(&mut self) -> &mut Vec<StringElement> { &mut self.manufacturer }
    
    pub fn get_frequency(&self) -> &Vec<FloatElement> { &self.frequency }
    pub fn get_frequency_mut(&mut self) -> &mut Vec<FloatElement> { &mut self.frequency }
    
    pub fn get_instruction_set(&self) -> &Vec<BasicElement<InstructionSet>>{ &self.instruction_set }
    pub fn get_instruction_set_mut(&mut self) -> &mut Vec<BasicElement<InstructionSet>> { &mut self.instruction_set }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryRepo {
    size: Vec<FloatElement>,
    generation: Vec<BasicElement<MemoryGeneration>>
}

impl MemoryRepo {
    
    pub fn get_size(&self) -> &Vec<FloatElement> { &self.size }
    pub fn get_size_mut(&mut self) -> &mut Vec<FloatElement> { &mut self.size }
    
    pub fn get_generation(&self) -> &Vec<BasicElement<MemoryGeneration>> { &self.generation }
    pub fn get_generation_mut(&mut self) -> &mut Vec<BasicElement<MemoryGeneration>> { &mut self.generation }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskRepo {
    disk_type: Vec<BasicElement<DiskType>>
}

impl DiskRepo {
    pub fn get_disk_type(&self) -> &Vec<BasicElement<DiskType>> { &self.disk_type }
    pub fn get_disk_type_mut(&mut self) -> &mut Vec<BasicElement<DiskType>> { &mut self.disk_type }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    magnitude: Vec<MagnitudeElement>,
    operating_system: OperatingSystemRepo,
    cpu: CPURepo,
    memory: MemoryRepo,
    disk: DiskRepo,
    virtualization: VirtualizationRepo
}

impl Repository {
    pub fn get_magnitude(&self) -> &Vec<MagnitudeElement> { &self.magnitude }
    pub fn get_magnitude_mut(&mut self) -> &mut Vec<MagnitudeElement> { &mut self.magnitude }

    pub fn get_operating_system(&self) -> &OperatingSystemRepo { &self.operating_system }
    pub fn get_operating_system_mut(&mut self) -> &mut OperatingSystemRepo { &mut self.operating_system }

    pub fn get_cpu(&self) -> &CPURepo { &self.cpu }
    pub fn get_cpu_mut(&mut self) -> &mut CPURepo { &mut self.cpu }

    pub fn get_memory(&self) -> &MemoryRepo { &self.memory }
    pub fn get_memory_mut(&mut self) -> &mut MemoryRepo { &mut self.memory }

    pub fn get_disk(&self) -> &DiskRepo { &self.disk }
    pub fn get_disk_mut(&mut self) -> &mut DiskRepo { &mut self.disk }
    
    pub fn get_virtualization(&self) -> &VirtualizationRepo { &self.virtualization }
    pub fn get_virtualization_mut(&mut self) -> &mut VirtualizationRepo { &mut self.virtualization }
}

impl Repository {


    /**
    This function imports a file containing an instance
    of the Repository struct decoded as JSON dictionary.
    `Parameters`
      path:&str => Path to a file containing a repository
    `Return`
      An instance of the Repository struct
    **/
    pub fn from_json_file(path:&str) -> Repository {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let rep: Repository = serde_json::from_str(&data).unwrap();

        rep
    }

    /**
    This function imports a file containing an instance
    of the Offers struct decoded as MsgPack dictionary.
    `Parameters`
      path:&str => Path to a file containing a repository
    `Return`
      An instance of the Repository struct
    **/
    pub fn from_msgpack_file(path:&str) -> Repository {

        let mut msgpack_file: File = File::open(path).unwrap();
        let mut msgpack_data = Vec::new();

        msgpack_file.read_to_end(&mut msgpack_data).unwrap();
        let rep: Repository = rmps::decode::from_slice(&msgpack_data).unwrap();
        rep
    }

    pub fn to_msgpack_file(path:&str, mut repo:&Repository) {
        let mut file_new = File::create(path).unwrap();

        let mut new_data = Vec::new();

        new_data = rmps::encode::to_vec(&mut repo).unwrap();

        use std::io::Write;
        file_new.write_all(&new_data);
    }

}