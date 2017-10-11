/**
    Contains the different basic elements
*/
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std;


use serde_json::{Value, Error};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Magnitude {
    pico,
    nano,
    micro,
    milli,
    none,
    kilo,
    mega,
    giga,
    tera,
    peta
}

impl Default for Magnitude {
    fn default() -> Magnitude { Magnitude::none }
}

impl Magnitude {
    fn to_float(&self) -> f64 {
        match self {
            &Magnitude::pico => 10_f64.powf(-12.0),
            &Magnitude::nano => 10_f64.powf(-9.0),
            &Magnitude::micro => 10_f64.powf(-6.0),
            &Magnitude::milli => 10_f64.powf(-3.0),
            &Magnitude::none => 1_f64,
            &Magnitude::kilo => 10_f64.powf(3.0),
            &Magnitude::mega => 10_f64.powf(6.0),
            &Magnitude::giga => 10_f64.powf(9.0),
            &Magnitude::tera => 10_f64.powf(12.0),
            &Magnitude::peta => 10_f64.powf(15.0),
        }
    }
}

impl PartialEq for Magnitude {
    fn eq(&self, other: &Magnitude) -> bool {
        match (self, other) {
            (&Magnitude::pico, &Magnitude::pico) => true,
            (&Magnitude::nano, &Magnitude::nano) => true,
            (&Magnitude::micro, &Magnitude::micro) => true,
            (&Magnitude::milli, &Magnitude::milli) => true,
            (&Magnitude::none, &Magnitude::none) => true,
            (&Magnitude::kilo, &Magnitude::kilo) => true,
            (&Magnitude::mega, &Magnitude::mega) => true,
            (&Magnitude::giga, &Magnitude::giga) => true,
            (&Magnitude::tera, &Magnitude::tera) => true,
            (&Magnitude::peta, &Magnitude::peta) => true,
            (_, _) => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MagnitudeElement {
    id: Option<i16>,
    magnitude: Option<Magnitude>,
}

impl Default for MagnitudeElement {
    fn default() -> MagnitudeElement {
        MagnitudeElement { id: None, magnitude: None }
    }
}

impl MagnitudeElement {
    pub fn get_id(&self) -> &Option<i16> { &self.id }

    pub fn get_id_mut(&mut self) -> &mut Option<i16> { &mut self.id }

    pub fn set_id(&mut self, id: i16) { self.id = Some(id); }

    pub fn get_magnitude(&self) -> &Option<Magnitude> {
        &self.magnitude
    }
    pub fn get_magnitude_mut(&mut self) -> &mut Option<Magnitude> {
        &mut self.magnitude
    }

    pub fn set_magnitude(&mut self, mag: Option<Magnitude>) {
        self.magnitude = mag;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BasicElement<T> {
    id: Option<i16>,
    value: Option<T>,
}



impl <T: PartialEq> BasicElement<T> {


     pub fn compare(&self, b: &BasicElement<T>) -> bool {
         self.eq(b)
    }

    pub fn get_id(&self) -> &Option<i16> { &self.id }
    pub fn get_id_mut(&mut self) -> &mut Option<i16> { &mut self.id }
    pub fn set_id(&mut self, id: i16) { self.id = Some(id); }

    pub fn get_value(&self) -> &Option<T> {
        &self.value
    }
    pub fn get_value_mut(&mut self) -> &mut Option<T> {
        &mut self.value
    }
    pub fn set_value(&mut self, val: Option<T>) {
        self.value = val;
    }

}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StringElement {
    id: Option<i16>,
    value: Option<String>,
    #[serde(default)]
    compareOperator: StringOperator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StringOperator {
    eq,
    lowerCaseEq,
}

impl std::str::FromStr for StringOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<StringOperator, ()> {
        match s {
            "eq" => Ok(StringOperator::eq),
            "lowerCaseEq" => Ok(StringOperator::lowerCaseEq),
            _ => {
                println!("Error in String to StringOperator parsing");
                Ok(StringOperator::eq)
            }
        }
    }
}


impl Default for StringOperator {
    fn default() -> StringOperator { StringOperator::eq }
}

impl StringElement {
    pub fn get_id(&self) -> &Option<i16> { &self.id }
    pub fn get_id_mut(&mut self) -> &mut Option<i16> { &mut self.id }
    pub fn set_id(&mut self, id: i16) { self.id = Some(id); }

    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }
    pub fn get_value_mut(&mut self) -> &mut Option<String> {
        &mut self.value
    }
    pub fn set_value(&mut self, str: Option<String>) {
        self.value = str;
    }

    pub fn get_compare_operator(&self) -> &StringOperator {
        &self.compareOperator
    }
    pub fn get_compare_operator_mut(&mut self) -> &mut StringOperator {
        &mut self.compareOperator
    }
    pub fn set_compare_operator(&mut self, op: StringOperator) {
        self.compareOperator = op;
    }

    fn lower_case_eq(&self, b: &StringElement) -> bool {
        self.value.clone().unwrap().to_lowercase() == b.value.clone().unwrap().to_lowercase()
    }


    fn eq(&self, b: &StringElement) -> bool {
        self.value == b.value
    }

    pub fn compare(&self, b: &StringElement) -> bool {
        match self.compareOperator {
            StringOperator::lowerCaseEq => self.lower_case_eq(b),
            StringOperator::eq => self.eq(b),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IntElement {
    id: Option<i16>,
    value: Option<u32>,
    #[serde(default)]
    magnitude: MagnitudeElement,
    #[serde(default)]
    compareOperator: IntOperator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IntOperator {
    leq,
    geq,
    eq,
    le,
    ge,
}

impl std::str::FromStr for IntOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<IntOperator, ()> {
        match s {
            "leq" => Ok(IntOperator::leq),
            "geq" => Ok(IntOperator::geq),
            "eq" => Ok(IntOperator::eq),
            "le" => Ok(IntOperator::le),
            "ge" => Ok(IntOperator::ge),
            _ => {
                println!("Error in String to StringOperator parsing");
                Ok(IntOperator::eq)
            }
        }
    }
}


impl IntElement {
    fn leq(&self, b: &IntElement) -> bool {
         match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() <= b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() <= b.get_value().unwrap().clone() as f64,
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 <= b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone() as f64 <= b.get_value().unwrap().clone() as f64
        }
    }

    fn geq(&self, b: &IntElement) -> bool {
         match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() >= b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() >= b.get_value().unwrap().clone() as f64,
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 >= b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone() as f64 >= b.get_value().unwrap().clone() as f64
        }
    }

    fn eq(&self, b: &IntElement) -> bool {
         match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() == b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() == b.get_value().unwrap().clone() as f64,
            (&None, &Some(ref magnitude_b)) => (self.get_value().unwrap().clone() as f64) == b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&None, &None) => (self.get_value().unwrap().clone() as f64) == b.get_value().unwrap().clone() as f64
        }
    }

    fn le(&self, b: &IntElement) -> bool {
         match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() < b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() < b.get_value().unwrap().clone() as f64,
             (&None, &Some(ref magnitude_b)) => (self.get_value().unwrap().clone() as f64) < (b.get_value().unwrap().clone() as f64 * magnitude_b.to_float()),
            (&None, &None) => (self.get_value().unwrap().clone() as f64) < (b.get_value().unwrap().clone() as f64)
        }
    }

    fn ge(&self, b: &IntElement) -> bool {
         match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() > b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone() as f64 * magnitude_a.to_float() > b.get_value().unwrap().clone() as f64,
            (&None, &Some(ref magnitude_b)) => (self.get_value().unwrap().clone() as f64) > b.get_value().unwrap().clone() as f64 * magnitude_b.to_float(),
            (&None, &None) => (self.get_value().unwrap().clone() as f64) > b.get_value().unwrap().clone() as f64
        }
    }


    pub fn get_id(&self) -> &Option<i16> { &self.id }
    pub fn get_id_mut(&mut self) -> &mut Option<i16> { &mut self.id }
    pub fn set_id(&mut self, id: i16) { self.id = Some(id); }

    pub fn get_value(&self) -> &Option<u32> {
        &self.value
    }
    pub fn get_value_mut(&mut self) -> &mut Option<u32> {
        &mut self.value
    }
    pub fn set_value(&mut self, val: Option<u32>) {
        self.value = val;
    }

    pub fn get_compare_operator(&self) -> &IntOperator {
        &self.compareOperator
    }
    pub fn get_compare_operator_mut(&mut self) -> &mut IntOperator {
        &mut self.compareOperator
    }
    pub fn set_compare_operator(&mut self, op: IntOperator) {
        self.compareOperator = op;
    }

    pub fn get_magnitude(&self) -> &MagnitudeElement {
        &self.magnitude
    }
    pub fn get_magnitude_mut(&mut self) -> &mut MagnitudeElement {
        &mut self.magnitude
    }
    pub fn set_magnitude(&mut self, val: MagnitudeElement) {
        self.magnitude = val;
    }

    pub fn compare(&self, b: &IntElement) -> bool {
        match self.compareOperator {
            IntOperator::leq => self.leq(b),
            IntOperator::geq => self.geq(b),
            IntOperator::eq => self.eq(b),
            IntOperator::le => self.le(b),
            IntOperator::ge => self.ge(b)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FloatElement {
    id: Option<i16>,
    value: Option<f64>,
    #[serde(default)]
    magnitude: MagnitudeElement,
    #[serde(default)]
    compareOperator: FloatOperator,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FloatOperator {
    leq,
    geq,
    eq,
    le,
    ge,
}

impl Default for FloatOperator {
    fn default() -> FloatOperator { FloatOperator::eq }
}

impl Default for IntOperator {
    fn default() -> IntOperator { IntOperator::eq }
}


impl std::str::FromStr for FloatOperator {
    type Err = ();

    fn from_str(s: &str) -> Result<FloatOperator, ()> {
        match s {
            "leq" => Ok(FloatOperator::leq),
            "geq" => Ok(FloatOperator::geq),
            "eq" => Ok(FloatOperator::eq),
            "le" => Ok(FloatOperator::le),
            "ge" => Ok(FloatOperator::ge),
            _ => {
                println!("Error in String to StringOperator parsing");
                Ok(FloatOperator::eq)
            }
        }
    }
}

impl FloatElement {
    fn leq(&self, b: &FloatElement) -> bool {
        match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => (&self.get_value().unwrap()) * &magnitude_a.to_float() <= b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone() * magnitude_a.to_float() <= b.get_value().unwrap().clone(), 
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone() <= b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone()<= b.get_value().unwrap().clone()
        }
    }

    fn geq(&self, b: &FloatElement) -> bool {
        match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone()* magnitude_a.to_float() >= b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone()* magnitude_a.to_float() >= b.get_value().unwrap().clone(),
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone()>= b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone()>= b.get_value().unwrap().clone()
        }
    }

    fn eq(&self, b: &FloatElement) -> bool {
         match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone()* magnitude_a.to_float() == b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone()* magnitude_a.to_float() == b.get_value().unwrap().clone(),
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone()== b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone()== b.get_value().unwrap().clone()
        }
    }

    fn le(&self, b: &FloatElement) -> bool {
          match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone()* magnitude_a.to_float() < b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone()* magnitude_a.to_float() < b.get_value().unwrap().clone(),
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone()< b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone()< b.get_value().unwrap().clone()
        }
    }

    fn ge(&self, b: &FloatElement) -> bool {
          match (self.get_magnitude().get_magnitude(), b.get_magnitude().get_magnitude()) {
            (&Some(ref magnitude_a), &Some(ref magnitude_b)) => self.get_value().unwrap().clone()* magnitude_a.to_float() > b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&Some(ref magnitude_a), &None) => self.get_value().unwrap().clone()* magnitude_a.to_float() > b.get_value().unwrap().clone(),
            (&None, &Some(ref magnitude_b)) => self.get_value().unwrap().clone()> b.get_value().unwrap().clone() * magnitude_b.to_float(),
            (&None, &None) => self.get_value().unwrap().clone() > b.get_value().unwrap().clone()
        }
    }
    pub fn compare(&self, b: &FloatElement) -> bool {
        match self.compareOperator {
            FloatOperator::leq => self.leq(b),
            FloatOperator::geq => self.geq(b),
            FloatOperator::eq => self.eq(b),
            FloatOperator::le => self.le(b),
            FloatOperator::ge => self.ge(b)
        }
    }


    pub fn get_id(&self) -> &Option<i16> { &self.id }

    pub fn get_id_mut(&mut self) -> &mut Option<i16> { &mut self.id }

    pub fn set_id(&mut self, id: i16) { self.id = Some(id); }

    pub fn get_value(&self) -> &Option<f64> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Option<f64> {
        &mut self.value
    }

    pub fn set_value(&mut self, val: Option<f64>) {
        self.value = val;
    }

    pub fn get_compare_operator(&self) -> &FloatOperator {
        &self.compareOperator
    }

    pub fn get_compare_operator_mut(&mut self) -> &mut FloatOperator {
        &mut self.compareOperator
    }

    pub fn set_compare_operator(&mut self, op: FloatOperator) {
        self.compareOperator = op;
    }

    pub fn get_magnitude(&self) -> &MagnitudeElement {
        &self.magnitude
    }

    pub fn get_magnitude_mut(&mut self) -> &mut MagnitudeElement {
        &mut self.magnitude
    }

    pub fn set_magnitude(&mut self, val: MagnitudeElement) {
        self.magnitude = val;
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DiskType {
    HDD,
    SSD
}

impl std::str::FromStr for DiskType {
    type Err = ();

    fn from_str(s: &str) -> Result<DiskType, ()> {
        match s {
            "HDD" => Ok(DiskType::HDD),
            "SSD" => Ok(DiskType::SSD),
            _ => {
                println!("Error in String to DiskType parsing");
                Ok(DiskType::HDD)
            }
        }
    }
}

impl PartialEq for DiskType {
    fn eq(&self, other: &DiskType) -> bool {
        match (self, other) {
            (&DiskType::HDD, &DiskType::HDD) => true,
            (&DiskType::SSD, &DiskType::SSD) => true,
            (_, _) => false,
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InstructionSet {
    MMX,
    SSE,
    SSE2,
    SSE3,
    SSSE3,
    SSE4a,
    SSE4_1,
    SSE4_2,
    AVX,
    AVX2,
    FMA3,
    F16C,
    AES,
    AES_NI,
    BMI1,
    BMI2
}

impl std::str::FromStr for InstructionSet {
    type Err = ();

    fn from_str(s: &str) -> Result<InstructionSet, ()> {
        match s {
            "MMX" => Ok(InstructionSet::MMX),
            "SSE" => Ok(InstructionSet::SSE),
            "SSE2" => Ok(InstructionSet::SSE2),
            "SSE3" => Ok(InstructionSet::SSE3),
            "SSSE3" => Ok(InstructionSet::SSSE3),
            "SSE4a" => Ok(InstructionSet::SSE4a),
            "SSE4.1" => Ok(InstructionSet::SSE4_1),
            "SSE4.2" => Ok(InstructionSet::SSE4_2),
            "AVX" => Ok(InstructionSet::AVX),
            "AVX2" => Ok(InstructionSet::AVX2),
            "FMA3" => Ok(InstructionSet::FMA3),
            "F16C" => Ok(InstructionSet::F16C),
            "AES" => Ok(InstructionSet::AES),
            "AES-NI" => Ok(InstructionSet::AES_NI),
            "BMI1" => Ok(InstructionSet::BMI1),
            "BMI2" => Ok(InstructionSet::BMI2),
            _ => {
                println!("Error in String to MemoryGeneration parsing");
                Ok(InstructionSet::MMX)
            }
        }
    }
}

impl PartialEq for InstructionSet {
    fn eq(&self, other: &InstructionSet) -> bool {
        match (self, other) {
            (&InstructionSet::MMX, &InstructionSet::MMX) => true,
            (&InstructionSet::SSE, &InstructionSet::SSE) => true,
            (&InstructionSet::SSE2, &InstructionSet::SSE2) => true,
            (&InstructionSet::SSE3, &InstructionSet::SSE3) => true,
            (&InstructionSet::SSSE3, &InstructionSet::SSSE3) => true,
            (&InstructionSet::SSE4a, &InstructionSet::SSE4a) => true,
            (&InstructionSet::SSE4_1, &InstructionSet::SSE4_1) => true,
            (&InstructionSet::SSE4_2, &InstructionSet::SSE4_2) => true,
            (&InstructionSet::AVX, &InstructionSet::AVX) => true,
            (&InstructionSet::AVX2, &InstructionSet::AVX2) => true,
            (&InstructionSet::FMA3, &InstructionSet::FMA3) => true,
            (&InstructionSet::F16C, &InstructionSet::F16C) => true,
            (&InstructionSet::AES, &InstructionSet::AES) => true,
            (&InstructionSet::AES_NI, &InstructionSet::AES_NI) => true,
            (&InstructionSet::BMI1, &InstructionSet::BMI1) => true,
            (&InstructionSet::BMI2, &InstructionSet::BMI2) => true,
            (_, _) => false,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MemoryGeneration {
    DDRRAM,
    DDR2RAM,
    DDR3RAM,
    DDR4RAM
}

impl PartialEq for MemoryGeneration {
    fn eq(&self, other: &MemoryGeneration) -> bool {
        match (self, other) {
            (&MemoryGeneration::DDRRAM, &MemoryGeneration::DDRRAM) => true,
            (&MemoryGeneration::DDR2RAM, &MemoryGeneration::DDR2RAM) => true,
            (&MemoryGeneration::DDR3RAM, &MemoryGeneration::DDR3RAM) => true,
            (&MemoryGeneration::DDR4RAM, &MemoryGeneration::DDR4RAM) => true,
            (_, _) => false,
        }
    }
}

impl PartialOrd for MemoryGeneration {
    fn partial_cmp(&self, other: &MemoryGeneration) -> Option<Ordering> {
        match (self, other) {
            (&MemoryGeneration::DDRRAM, &MemoryGeneration::DDR2RAM) => Some(Ordering::Less),
            (&MemoryGeneration::DDRRAM, &MemoryGeneration::DDR3RAM) => Some(Ordering::Less),
            (&MemoryGeneration::DDRRAM, &MemoryGeneration::DDR4RAM) => Some(Ordering::Less),
            (&MemoryGeneration::DDR2RAM, &MemoryGeneration::DDR3RAM) => Some(Ordering::Less),
            (&MemoryGeneration::DDR3RAM, &MemoryGeneration::DDR4RAM) => Some(Ordering::Less),
            (&MemoryGeneration::DDRRAM, &MemoryGeneration::DDRRAM) => Some(Ordering::Equal),
            (&MemoryGeneration::DDR2RAM, &MemoryGeneration::DDR2RAM) => Some(Ordering::Equal),
            (&MemoryGeneration::DDR3RAM, &MemoryGeneration::DDR3RAM) => Some(Ordering::Equal),
            (&MemoryGeneration::DDR4RAM, &MemoryGeneration::DDR4RAM) => Some(Ordering::Equal),
            (_, _) => None,
        }
    }
}