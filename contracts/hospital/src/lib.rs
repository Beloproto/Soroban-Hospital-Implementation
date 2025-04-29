#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, String, Vec};


// Hospital contract Outline 

// Admin 

// Patient Managment Functions 
// -> register a patient 
// -> get a patient information 
// -> update patient record 
// -> set patient active 
// -> list all patients 

// Doctor management Functions 
// -> register a doctor 
// -> get Doctor Information 
// -> update Doctor information 
// -> set doctor active 
// -> list all Doctors 

// Mecdical Test Management functions 
// -> record medical test 
// -> get medical test (Doctor and patient) 
// -> get all medical test for a patient 
// -> get all medical test performed by a doctor 
// -> Statistics records for test and which department requested for test 
// -> list of medical tests 

// Things to take note 
// Data is stored 
// How to retreive data and store data 
// How to get environment variables 



#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Patient {
    id: u64,
    name: String,
    date_of_birth: u64,
    blood_type: String,
    allergies: Vec<String>,
    insurance_id: String,
    active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Doctor {
    id: u64,
    name: String,
    specialization: String,
    license_number: String, 
    active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MedicalTest {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    test_type: String,
    test_date: u64,
    results: String,
    notes: String,
}


 // pateient : id -> name -> PatientData
 // id -> PatientData 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Patient(u64)
}


#[contract]
pub struct HospitalContract;

#[contractimpl]
impl HospitalContract {
   fn initialize() {
    todo!()
   }
}

mod test;
