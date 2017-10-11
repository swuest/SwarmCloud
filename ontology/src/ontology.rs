/**
This module contains the structures to define the ontology,
as well as functions to compress or compare offers with each other.

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


use basicElements::{BasicElement,IntElement,FloatElement,StringElement,MagnitudeElement};
use basicElements::{InstructionSet, DiskType,MemoryGeneration};
use serde_json::{Value, Error};

use repository::Repository;


enum ontology_errors {
    compare_error(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    name: Option<StringElement>,
    version: Option<StringElement>,
}

impl Application {
    fn compare(&self, app: &Application) -> bool {
        let mut result: bool = true;

        if let Some(ref name_a) = self.name {
            if let Some(ref name_b) = app.name {
                result &= name_a.compare(name_b);
            }
        }

        if let Some(ref version_a) = self.version {
            if let Some(ref version_b) = app.version {
                result &= version_a.compare(version_b);
            }
        }

        result
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {}
    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CPU {
    manufacturer: Option<StringElement>,
    amount: Option<IntElement>,
    frequency: Option<FloatElement>,
    instruction_set: Vec<BasicElement<InstructionSet>>
}



impl CPU {
    fn compress(&mut self, repo: &Repository) {
        let frequencies = repo.get_cpu().get_frequency();
        for cpu_frequency in frequencies {
            match self.frequency {
                Some(ref mut frequency) => {
                    if frequency.get_value() == cpu_frequency.get_value() {
                        frequency.set_id(cpu_frequency.get_id().unwrap().clone());
                        frequency.set_value(None);
                    }
                }
                None => ()
            }
        }

        let manufacturers = repo.get_cpu().get_manufacturer();
        for cpu_manufacturer in manufacturers {
            match self.manufacturer {
                Some(ref mut manufacturer) => {
                    if manufacturer.get_value() == cpu_manufacturer.get_value() {
                        manufacturer.set_id(cpu_manufacturer.get_id().unwrap().clone());
                        manufacturer.set_value(None);
                    }
                }
                None => ()
            }
        }


        let instruction_set_extensions = repo.get_cpu().get_instruction_set();
        for cpu_instruction_sets in instruction_set_extensions {
            for mut instruction_set_element in self.instruction_set.iter_mut() {
                if instruction_set_element.get_value() == cpu_instruction_sets.get_value() {
                    instruction_set_element.set_id(cpu_instruction_sets.get_id().unwrap().clone());
                    instruction_set_element.set_value(None);
                }
            }
        }
    }

    fn decompress(&mut self, repo: &Repository) {
        let frequencies = repo.get_cpu().get_frequency();
        for cpu_frequency in frequencies {
            match self.frequency {
                Some(ref mut frequency) => {
                    if frequency.get_id() == cpu_frequency.get_id() {
                        frequency.set_value(cpu_frequency.get_value().clone());
                    }
                }
                None => ()
            }
        }

        let manufacturers = repo.get_cpu().get_manufacturer();
        for cpu_manufacturer in manufacturers {
            match self.manufacturer {
                Some(ref mut manufacturer) => {
                    if manufacturer.get_id() == cpu_manufacturer.get_id() {
                        manufacturer.set_value(cpu_manufacturer.get_value().clone());
                    }
                }
                None => ()
            }
        }


        let instruction_set_extensions = repo.get_cpu().get_instruction_set();
        for cpu_instruction_sets in instruction_set_extensions {
            for mut instruction_set_element in self.instruction_set.iter_mut() {
                if instruction_set_element.get_id() == cpu_instruction_sets.get_id() {
                    instruction_set_element.set_value(cpu_instruction_sets.get_value().clone());
                }
            }
        }
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.amount {
                Some(ref mut amount) => {
                    if amount.get_magnitude().clone().get_magnitude() == mg.get_magnitude() {
                        amount.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        amount.get_magnitude_mut().set_magnitude(None);
                    }
                }
                None => ()
            }
            match self.frequency {
                Some(ref mut frequency) => {
                    if frequency.get_magnitude().clone().get_magnitude() == mg.get_magnitude() {
                        frequency.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        frequency.get_magnitude_mut().set_magnitude(None);
                    }
                }
                None => ()
            }
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.amount {
                Some(ref mut amount) => {
                    if amount.get_magnitude().clone().get_id() == mg.get_id() {
                        amount.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        amount.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
                None => ()
            }
            match self.frequency {
                Some(ref mut frequency) => {
                    if frequency.get_magnitude().clone().get_id() == mg.get_id() {
                        frequency.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
                None => ()
            }
        }
    }

    fn compare(&self, b: &CPU) -> bool {
        let mut result: bool = true;
        if let Some(ref manufacturer_a) = self.manufacturer {
            if let Some(ref manufacturer_b) = b.manufacturer {
                result &= manufacturer_a.compare(manufacturer_b);
            }
        }
        if let Some(ref amount_a) = self.amount {
            if let Some(ref amount_b) = b.amount {
                result &= amount_a.compare(&amount_b);
            }
        }
        if let Some(ref frequency_a) = self.frequency {
            if let Some(ref frequency_b) = b.frequency {
                result &= frequency_a.compare(frequency_b);
            }
        }
        let mut subset: bool = true;
        for set_a in &self.instruction_set {
            let mut element_of: bool = false;
            for set_b in &b.instruction_set {
                if set_a.get_value() == set_b.get_value() {
                    element_of = true;
                }
            }
            subset &= element_of;
        }
        result &= subset;

        result
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    size: Option<FloatElement>,
    generation: Option<BasicElement<MemoryGeneration>>,
}



impl Memory {
    fn compress(&mut self, repo: &Repository) {
        let magnitude = repo.get_magnitude();
        self.compress_magnitude_elements(magnitude);


        let memory_sizes = repo.get_memory().get_size();
        match self.size {
            Some(ref mut size) => {
                for memory in memory_sizes {
                    if size.get_value() == memory.get_value() {
                        size.set_id(memory.get_id().unwrap().clone());
                        size.set_value(None);
                    }
                }
            }
            None => ()
        }

        let memory_generation = repo.get_memory().get_generation();
        match self.generation {
            Some(ref mut generation) => {
                for gen in memory_generation {
                    if generation.get_value() == gen.get_value() {
                        generation.set_id(gen.get_id().unwrap().clone());
                        generation.set_value(None);
                    }
                }
            }
            None => {}
        }
    }


    fn decompress(&mut self, repo: &Repository) {
        let magnitude = repo.get_magnitude();
        self.decompress_magnitude_elements(magnitude);


        let memory_sizes = repo.get_memory().get_size();
        match self.size {
            Some(ref mut size) => {
                for memory in memory_sizes {
                    if size.get_id() == memory.get_id() {
                        size.set_value(memory.get_value().clone());
                    }
                }
            }
            None => ()
        }

        let memory_generation = repo.get_memory().get_generation();
        match self.generation {
            Some(ref mut generation) => {
                for gen in memory_generation {
                    if generation.get_id() == gen.get_id() {
                        generation.set_value(gen.get_value().clone());
                    }
                }
            }
            None => {}
        }
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        match self.size {
            Some(ref mut size) => {
                for mg in mag {
                    if size.get_magnitude().clone().get_magnitude() == mg.get_magnitude() {
                        size.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        size.get_magnitude_mut().set_magnitude(None);
                    }
                }
            }
            None => ()
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        match self.size {
            Some(ref mut size) => {
                for mg in mag {
                    if size.get_magnitude().clone().get_id() == mg.get_id() {
                        size.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
            }
            None => ()
        }
    }

    fn compare(&self, b: &Memory) -> bool {
        let mut result: bool = true;
        if let Some(ref size_a) = self.size {
            if let Some(ref size_b) = b.size {
                result &= size_a.compare(&size_b);
            }
        }
        if let Some(ref generation_a) = self.generation {
            if let Some(ref generation_b) = b.generation {
                result &= generation_a.get_value() <= generation_b.get_value();
            }
        }

        result
    }
}


impl std::fmt::Display for MemoryGeneration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let generation = match *self {
            MemoryGeneration::DDRRAM => "DDR-RAM",
            MemoryGeneration::DDR2RAM => "DDR2-RAM",
            MemoryGeneration::DDR3RAM => "DDR3-RAM",
            MemoryGeneration::DDR4RAM => "DDR4-RAM",
        };
        write!(f, "{}", generation)
    }
}

impl std::str::FromStr for MemoryGeneration {
    type Err = ();

    fn from_str(s: &str) -> Result<MemoryGeneration, ()> {
        match s {
            "DDRRAM" => Ok(MemoryGeneration::DDRRAM),
            "DDR2RAM" => Ok(MemoryGeneration::DDR2RAM),
            "DDR3RAM" => Ok(MemoryGeneration::DDR3RAM),
            "DDR4RAM" => Ok(MemoryGeneration::DDR4RAM),
            _ => {
                println!("Error in String to MemoryGeneration parsing");
                Ok(MemoryGeneration::DDRRAM)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OperatingSystem {
    system_type: Option<StringElement>,
    repository: Option<StringElement>,
    version: Option<StringElement>,
    custom_template: Option<bool>,
}


impl OperatingSystem {
    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {}
    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {}

    fn compare(&self, b: &OperatingSystem) -> bool {
        let mut result: bool = true;
        if let Some(ref system_type_a) = self.system_type {
            if let Some(ref system_type_b) = b.system_type {
                result &= system_type_a.compare(system_type_b);
                if !result {
                    println!("- System Type is not the same");
                }
            }
        }
        if let Some(ref repository_a) = self.repository {
            if let Some(ref repository_b) = b.repository {
                result &= repository_a.compare(repository_b);
                if !result {
                    println!("- Repository is not the same");
                }
            }
        }

        if let Some(ref version_a) = self.version {
            if let Some(ref version_b) = b.version {
                result &= version_a.compare(version_b);
                if !result {
                    println!("- Version is not the same");
                }
            }
        }

        if let Some(ref custom_template_a) = self.custom_template {
            if let Some(ref custom_template_b) = b.custom_template {
                result &= custom_template_a == custom_template_b;
                if !result {
                    println!("- Custom Template is not the same");
                }
            }
        }

        result
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Disk {
    diskType: Option<BasicElement<DiskType>>,
    size: Option<IntElement>,
    performance: Option<DiskPerformance>,
}

impl Disk {
    fn compare(&self, dsk: &Disk) -> bool {
        let mut result: bool = true;

        if let Some(ref size_a) = self.size {
            if let Some(ref size_b) = dsk.size {
                result &= size_a.compare(size_b);
            }
        }

        if let Some(ref diskType_a) = self.diskType {
            if let Some(ref diskType_b) = dsk.diskType {
                result &= diskType_a.compare(diskType_b);
            }
        }

        if let Some(ref performance_a) = self.size {
            if let Some(ref performance_b) = dsk.size {
                result &= performance_a.compare(performance_b);
            }
        }
        result
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.size {
                Some(ref mut size) => {
                    if size.get_magnitude().clone().get_magnitude() == mg.get_magnitude() {
                        size.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        size.get_magnitude_mut().set_magnitude(None);
                    }
                }
                None => ()
            }
            match self.performance {
                Some(ref mut performance) => {
                    performance.compress_magnitude_elements(&mag);
                }
                None => ()
            }
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.size {
                Some(ref mut size) => {
                    if size.get_magnitude().clone().get_id() == mg.get_id() {
                        size.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
                None => ()
            }
            match self.performance {
                Some(ref mut performance) => {
                    performance.decompress_magnitude_elements(&mag);
                }
                None => ()
            }
        }
    }

    fn compress(&mut self, repo: &Repository) {
        let disk_types = repo.get_disk().get_disk_type();
        for disk_type in disk_types {
            match self.diskType {
                Some(ref mut diskType) => {
                    if diskType.get_value() == disk_type.get_value() {
                        diskType.set_id(disk_type.get_id().unwrap().clone());
                        diskType.set_value(None);
                    }
                }
                None => ()
            }
        }
    }

    fn decompress(&mut self, repo: &Repository) {
        let disk_types = repo.get_disk().get_disk_type();
        for disk_type in disk_types {
            match self.diskType {
                Some(ref mut diskType) => {
                    if diskType.get_id() == disk_type.get_id() {
                        diskType.set_value(disk_type.get_value().clone());
                    }
                }
                None => ()
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskPerformance {
    read_performance: Option<IntElement>,
    write_performance: Option<IntElement>,
}

impl DiskPerformance {
    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.read_performance {
                Some(ref mut read_performance) => {
                    if read_performance.get_magnitude().get_magnitude() == mg.get_magnitude() {
                        read_performance.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        read_performance.get_magnitude_mut().set_magnitude(None);
                    }
                }
                None => ()
            }
            match self.write_performance {
                Some(ref mut write_performance) => {
                    if write_performance.get_magnitude().get_magnitude() == mg.get_magnitude() {
                        write_performance.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        write_performance.get_magnitude_mut().set_magnitude(None);
                    }
                }
                None => ()
            }
        }
    }


    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.read_performance {
                Some(ref mut read_performance) => {
                    if read_performance.get_magnitude().get_id() == mg.get_id() {
                        read_performance.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
                None => ()
            }
            match self.write_performance {
                Some(ref mut write_performance) => {
                    if write_performance.get_magnitude().get_id() == mg.get_id() {
                        write_performance.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
                None => ()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkInterface {
    performance: Option<NetworkPerformance>,
    quota: Option<Quota>,
}

impl NetworkInterface {
    fn compare(&self, networkInterface: &NetworkInterface) -> bool {
        let mut result: bool = true;

        if let Some(ref performance_a) = self.performance {
            if let Some(ref performance_b) = networkInterface.performance {
                result &= performance_a.compare(performance_b);
            }
        }

        if let Some(ref quota_a) = self.quota {
            if let Some(ref quota_b) = networkInterface.quota {
                result &= quota_a.compare(quota_b);
            }
        }

        result
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        match self.performance {
            Some(ref mut performance) => {
                performance.compress_magnitude_elements(&mag);
            }
            None => ()
        }
        match self.quota {
            Some(ref mut quota) => {
                quota.compress_magnitude_elements(&mag);
            }
            None => ()
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        match self.performance {
            Some(ref mut performance) => {
                performance.decompress_magnitude_elements(&mag);
            }
            None => ()
        }
        match self.quota {
            Some(ref mut quota) => {
                quota.decompress_magnitude_elements(&mag);
            }
            None => ()
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkPerformance {
    download_speed: Option<IntElement>,
    upload_speed: Option<IntElement>,
}

impl NetworkPerformance {
    fn compare(&self, networkPerformance: &NetworkPerformance) -> bool {
        let mut result: bool = true;

        if let Some(ref download_speed_a) = self.download_speed {
            if let Some(ref download_speed_b) = networkPerformance.download_speed {
                result &= download_speed_a.compare(download_speed_b);
            }
        }

        if let Some(ref upload_speed_a) = self.upload_speed {
            if let Some(ref upload_speed_b) = networkPerformance.upload_speed {
                result &= upload_speed_a.compare(upload_speed_b);
            }
        }

        result
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        {
            for mg in mag {
                match self.download_speed {
                    Some(ref mut download_speed) => {
                        if download_speed.get_magnitude().get_magnitude() == mg.get_magnitude() {
                            download_speed.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                            download_speed.get_magnitude_mut().set_magnitude(None);
                        }
                    }
                    None => ()
                }
                match self.upload_speed {
                    Some(ref mut upload_speed) => {
                        if upload_speed.get_magnitude().get_magnitude() == mg.get_magnitude() {
                            upload_speed.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                            upload_speed.get_magnitude_mut().set_magnitude(None);
                        }
                    }
                    None => ()
                }
            }
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        {
            for mg in mag {
                match self.download_speed {
                    Some(ref mut download_speed) => {
                        if download_speed.get_magnitude().get_id() == mg.get_id() {
                            download_speed.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                        }
                    }
                    None => ()
                }
                match self.upload_speed {
                    Some(ref mut upload_speed) => {
                        if upload_speed.get_magnitude().get_id() == mg.get_id() {
                            upload_speed.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                        }
                    }
                    None => ()
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quota {
    size: Option<IntElement>,
    reset_interval: Option<IntElement>,
}


impl Quota {
    fn compare(&self, quota: &Quota) -> bool {
        let mut result: bool = true;

        if let Some(ref size_a) = self.size {
            if let Some(ref size_b) = quota.size {
                result &= size_a.compare(size_b);
            }
        }

        if let Some(ref reset_interval_a) = self.reset_interval {
            if let Some(ref reset_interval_b) = quota.reset_interval {
                result &= reset_interval_a.compare(reset_interval_b);
            }
        }

        result
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        {
            for mg in mag {
                match self.size {
                    Some(ref mut size) => {
                        if size.get_magnitude().get_magnitude() == mg.get_magnitude() {
                            size.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                            size.get_magnitude_mut().set_magnitude(None);
                        }
                    }
                    None => ()
                }
                match self.reset_interval {
                    Some(ref mut reset_interval) => {
                        if reset_interval.get_magnitude().get_magnitude() == mg.get_magnitude() {
                            reset_interval.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                            reset_interval.get_magnitude_mut().set_magnitude(None);
                        }
                    }
                    None => ()
                }
            }
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        {
            for mg in mag {
                match self.size {
                    Some(ref mut size) => {
                        if size.get_magnitude().get_id() == mg.get_id() {
                            size.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                        }
                    }
                    None => ()
                }
                match self.reset_interval {
                    Some(ref mut reset_interval) => {
                        if reset_interval.get_magnitude().get_id() == mg.get_id() {
                            reset_interval.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                        }
                    }
                    None => ()
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Virtualization {
    technology: Option<StringElement>,
    version: Option<IntElement>,
    virtualization_type: Option<StringElement>,
}

impl Virtualization {
    fn compare(&self, virt: &Virtualization) -> bool {
        let mut result: bool = true;

        if let Some(ref technology_a) = self.technology {
            if let Some(ref technology_b) = virt.technology {
                result &= technology_a.compare(technology_b);
            }
        }

        if let Some(ref version_a) = self.version {
            if let Some(ref version_b) = virt.version {
                result &= version_a.compare(version_b);
            }
        }

        if let Some(ref virtualization_type_a) = self.virtualization_type {
            if let Some(ref virtualization_type_b) = virt.virtualization_type {
                result &= virtualization_type_a.compare(virtualization_type_b);
            }
        }
        result
    }
    
    
    fn compress(&mut self, repo: &Repository) {
        let virtualization_types = repo.get_virtualization().get_virtualization_type();
        for virtualization_type in virtualization_types {
            match self.virtualization_type {
                Some(ref mut virtualizationType) => {
                    if virtualizationType.get_value() == virtualization_type.get_value() {
                        virtualizationType.set_id(virtualization_type.get_id().unwrap().clone());
                        virtualizationType.set_value(None);
                    }
                }
                None => ()
            }
        }
    }

    fn decompress(&mut self, repo: &Repository) {
        let virtualization_types = repo.get_virtualization().get_virtualization_type();
        for virtualization_type in virtualization_types {
            match self.virtualization_type {
                Some(ref mut virtualizationType) => {
                    if virtualizationType.get_id() == virtualization_type.get_id() {
                        virtualizationType.set_value(virtualization_type.get_value().clone());
                    }
                }
                None => ()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    price: Option<IntElement>,
    currency: Option<StringElement>,
}


impl Payment {
    fn compare(&self, payment: &Payment) -> bool {
        let mut result: bool = true;

        if let Some(ref price_a) = self.price {
            if let Some(ref price_b) = payment.price {
                result &= price_a.compare(price_b);
            }
        }

        if let Some(ref currency_a) = self.currency {
            if let Some(ref currency_b) = payment.currency {
                result &= currency_a.compare(currency_b);
            }
        }

        result
    }


    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.price {
                Some(ref mut price) => {
                    if price.get_magnitude().get_magnitude() == mg.get_magnitude() {
                        price.get_magnitude_mut().set_id(mg.get_id().unwrap().clone());
                        price.get_magnitude_mut().set_magnitude(None);
                    }
                }
                None => ()
            }
        }
    }

    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        for mg in mag {
            match self.price {
                Some(ref mut price) => {
                    if price.get_magnitude().get_id() == mg.get_id() {
                        price.get_magnitude_mut().set_magnitude(mg.get_magnitude().clone());
                    }
                }
                None => ()
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Offer {
    host: StringElement,
    ontology_version: FloatElement,
    repository_version: FloatElement,
    timeout: IntElement,
    operating_system: Option<OperatingSystem>,
    network_interface: Option<Vec<NetworkInterface>>,
    virtualization: Option<Virtualization>,
    cpu: Option<CPU>,
    memory: Option<Memory>,
    disk: Option<Vec<Disk>>,
    application: Option<Vec<Application>>,
    payment: Option<Payment>,
}

impl Offer {


    pub fn decompress(&mut self, repo: &Repository) {

        let magnitude = repo.get_magnitude();
        self.decompress_magnitude_elements(magnitude);
        match self.cpu {
            Some(ref mut cpu) => {
                cpu.decompress(repo);
            }
            None => ()
        }
        match self.memory {
            Some(ref mut memory) => {
                memory.decompress(repo);
            }
            None => ()
        }

        match self.disk {
            Some(ref mut disks) => {
                for mut disk in disks.iter_mut() {
                    disk.decompress(repo);
                }
            }
            None => ()
        }
    }


    pub fn compress(&mut self, repo: &Repository) {
        let magnitude = repo.get_magnitude();
        self.compress_magnitude_elements(magnitude);
        match self.cpu {
            Some(ref mut cpu) => {
                cpu.compress(repo);
            }
            None => ()
        }
        match self.memory {
            Some(ref mut memory) => {
                memory.compress(repo);
            }
            None => ()
        }

        match self.disk {
            Some(ref mut disks) => {
                for mut disk in disks.iter_mut() {
                    disk.compress(repo);
                }
            }
            None => ()
        }
    }

    fn compress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        match self.operating_system {
            Some(ref mut operating_system) => { operating_system.compress_magnitude_elements(&mag); }
            None => ()
        }
        match self.network_interface {
            Some(ref mut network_interfaces) => {
                for ref mut interface in network_interfaces {
                    interface.compress_magnitude_elements(&mag);
                }
            }
            None => ()
        }
        match self.cpu {
            Some(ref mut cpu) => { cpu.compress_magnitude_elements(&mag); }
            None => ()
        }

        match self.memory {
            Some(ref mut memory) => { memory.compress_magnitude_elements(&mag); }
            None => ()
        }

        match self.disk {
            Some(ref mut disks) => {
                for ref mut dsk in disks {
                    dsk.compress_magnitude_elements(&mag);
                }
            }
            None => ()
        }

        match self.application {
            Some(ref mut application) => {
                for ref mut app in application {
                    app.compress_magnitude_elements(&mag);
                }
            }
            None => ()
        }

        match self.payment {
            Some(ref mut payment) => { payment.compress_magnitude_elements(&mag); }
            None => ()
        }
    }
    
    fn decompress_magnitude_elements(&mut self, mag: &Vec<MagnitudeElement>) {
        match self.operating_system {
            Some(ref mut operating_system) => { operating_system.decompress_magnitude_elements(&mag); }
            None => ()
        }
        match self.network_interface {
            Some(ref mut network_interfaces) => {
                for ref mut interface in network_interfaces {
                    interface.decompress_magnitude_elements(&mag);
                }
            }
            None => ()
        }
        match self.cpu {
            Some(ref mut cpu) => { cpu.decompress_magnitude_elements(&mag); }
            None => ()
        }

        match self.memory {
            Some(ref mut memory) => { memory.decompress_magnitude_elements(&mag); }
            None => ()
        }

        match self.disk {
            Some(ref mut disks) => {
                for ref mut dsk in disks {
                    dsk.decompress_magnitude_elements(&mag);
                }
            }
            None => ()
        }

        match self.application {
            Some(ref mut application) => {
                for ref mut app in application {
                    app.decompress_magnitude_elements(&mag);
                }
            }
            None => ()
        }

        match self.payment {
            Some(ref mut payment) => { payment.decompress_magnitude_elements(&mag); }
            None => ()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Offers {
    offers: Vec<Offer>
}

impl Offers {

    pub fn get_offers(&self) -> &Vec<Offer> {
        &self.offers
    }
    pub fn get_offers_mut(&mut self) -> &mut Vec<Offer> {
        &mut self.offers
    }


    /**
    This function imports a file containing an instance
    of the Offers struct decoded as JSON dictionary.
    `Parameters`
      path:&str => Path to a file containing the offerlist
    `Return`
      An instance of the Offers struct
    **/
    pub fn from_json_file(path:&str) -> Offers {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let mut off: Offers = serde_json::from_str(&data).unwrap();

        off
    }

    pub fn to_msgpack_file(path:&str, mut off:&Offers) {
        let mut file_new = File::create(path).unwrap();
        let mut new_data = rmps::encode::to_vec(&mut off).unwrap();

        use std::io::Write;
        file_new.write_all(&new_data);
    }

    /**
    This function imports a file containing an instance
    of the Offers struct decoded as MsgPack dictionary.
    `Parameters`
      path:&str => Path to a file containing an offerlist
    `Return`
      An instance of the Offers struct
    **/
    pub fn from_msgpack_file(path:&str) -> Offers {

        let mut msgpack_file: File = File::open(path).unwrap();
        let mut msgpack_data = Vec::new();

        msgpack_file.read_to_end(&mut msgpack_data).unwrap();
        let o: Offers = rmps::decode::from_slice(&msgpack_data).unwrap();
        o
    }


    /**
    Compresses all containing offers by using the repository and setting the Id fields.
    Warning: Replaces all values with None.
    `Parameters`
    * repo:&Repository: The repository which should be used to compress.
    `Return`
      None
    */
    pub fn compress(&mut self, repo:&Repository) {
        for of in &mut self.offers {
            {
                of.compress(repo);
            }
        }
    }

    /**
    Decompresses all containing offers by using the repository and setting the value fields
    according to the IDs.
    `Parameters`
    * repo:&Repository: The repository which should be used to decompress.
    `Return`
      None
    */
    pub fn decompress(&mut self, repo:&Repository) {
        for of in &mut self.offers {
            {
                of.decompress(repo);
            }
        }
    }
}

/**
Compares two offers with each other. It uses the individual
comparison functions defined in each element to check if
offer b is sufficient for an request defined in offer a.
`Parameters`
* a:&Offer: Offer used as request
* b:&Offer: Offer which is compared to the request
`Return`
 Ok(bool):
    True if b is sufficient for the request a,
    False if b is not sufficient.
*/
pub fn comparing(a: &Offer, b: &Offer) -> Result<bool, Error> {
    let mut result: bool = true;

    result &= a.host.compare(&b.host);
    if !result {
        println!("Hosts are not equal");
    }
    result &= a.ontology_version.compare(&b.ontology_version);
    if !result {
        println!("Versions are not equal");
    }

    result &= a.repository_version.compare(&b.repository_version);
    if !result {
        println!("Versions are not equal");
    }

    if let Some(ref cpu_a) = a.cpu {
        if let Some(ref cpu_b) = b.cpu {
            result &= cpu_a.compare(cpu_b);
            if !result {
                println!("cpu are not equal");
            }
        }
    }

    if let Some(ref memory_a) = a.memory {
        if let Some(ref memory_b) = b.memory {
            result &= memory_a.compare(memory_b);
            if !result {
                println!("memory are not equal");
            }
        }
    }

    if let Some(ref operating_system_a) = a.operating_system {
        if let Some(ref operating_system_b) = b.operating_system {
            result &= operating_system_a.compare(operating_system_b);
            if !result {
                println!("operating_system are not equal");
            }
        }
    }
    if let Some(ref disks_a) = a.disk {
        if let Some(ref disks_b) = b.disk {
            for ref disk_a in disks_a {
                let mut one_compatible: bool = false;
                for ref disk_b in disks_b {
                    one_compatible |= disk_a.compare(disk_b);
                }
                result &= one_compatible;
                if !result {
                    println!("operating_system are not equal");
                }
            }
        }
    }

    if let Some(ref network_interfaces_a) = a.network_interface {
        if let Some(ref network_interfaces_b) = b.network_interface {
            for ref network_interface_a in network_interfaces_a {
                let mut one_compatible: bool = false;
                for ref network_interface_b in network_interfaces_b {
                    one_compatible |= network_interface_a.compare(network_interface_b);
                }
                result &= one_compatible;
                if !result {
                    println!("network interfaces are not equal");
                }
            }
        }
    }

    if let Some(ref virtualization_a) = a.virtualization {
        if let Some(ref virtualization_b) = b.virtualization {
            result &= virtualization_a.compare(virtualization_b);
            if !result {
                println!("virtualization are not equal");
            }
        }
    }

   if let Some(ref applications_a) = a.application {
        if let Some(ref applications_b) = b.application {
            for ref application_a in applications_a {
                let mut one_compatible: bool = false;
                for ref application_b in applications_b {
                    one_compatible |= application_a.compare(application_b);
                }
                result &= one_compatible;
                if !result {
                    println!("applications are not equal");
                }
            }
        }
    }

    if let Some(ref payment_a) = a.payment {
        if let Some(ref payment_b) = b.payment {
            result &= payment_a.compare(payment_b);
            if !result {
                println!("payment are not equal");
            }
        }
    }

    Ok(result)
}
