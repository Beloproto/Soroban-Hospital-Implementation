#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Address, Env, String, Vec};


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
// Structure of data  
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
 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Patient(u64),
    Doctor(u64),
    MedicalTest(u64),
    PatientTests(u64),
    DoctorTests(u64),
    PatientCount,
    DoctorCount,
    TestCount,
}


#[contract]
pub struct HospitalContract;

#[contractimpl]
impl HospitalContract {

    // Initializer -> It initialize the contract with the admin 
   pub fn initialize(env: Env, admin: Address) -> Address {

    if env.storage().instance().has(&DataKey::Admin) {
        panic!("Contract already initilized");
    }

    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().set(&DataKey::PatientCount, &0u64);
    env.storage().instance().set(&DataKey::DoctorCount, &0u64);
    env.storage().instance().set(&DataKey::TestCount, &0u64);

    admin
   }

   fn check_admin (env: &Env) {
    let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
    admin.require_auth();
   }

   // Patient Managment Functions 

   pub fn register_patient (
    env: Env,
    name: String,
    date_of_birth: u64,
    blood_type: String,
    allergies: Vec<String>,
    insurance_id: String,
   ) -> u64 {
        Self::check_admin(&env);

        // Get and increment patient count
        let patient_count: u64 = env.storage().instance().get(&DataKey::PatientCount).unwrap_or(0);
        let new_id = patient_count + 1;

        // create the patiet record 
        let patient = Patient {
            id: new_id,
            name,
            date_of_birth,
            blood_type,
            allergies,
            insurance_id,
            active: true,
        };
        
        // Store the patient data and update update the count 

        env.storage().instance().set(&DataKey::Patient(new_id), &patient);
        env.storage().instance().set(&DataKey::PatientCount, &new_id);

        // Initialize empty test list for patient 

        env.storage().instance().set(&DataKey::PatientTests(new_id), &Vec::<u64>::new(&env));

        new_id
   }

   // Get patient information 

   pub fn get_patient(env: Env, id: u64) -> Patient {
        match env.storage().instance().get(&DataKey::Patient(id)) {
            Some(patient) => patient,
            None => panic!("Patient not registered"),
        }
   }


   // Update patient record 
   pub fn update_patient (
    env: Env,
    id: u64,
    name: String,
    date_of_birth: u64,
    blood_type: String,
    allergies: Vec<String>,
    insurance_id: String
   ) -> Patient {
    Self::check_admin(&env);

    // Get existing patient 
    let mut patient: Patient = env.storage().instance().get(&DataKey::Patient(id)).
        unwrap_or_else(|| panic!("Patient not found"));


    // Update fields 
    patient.name = name;
    patient.date_of_birth = date_of_birth;
    patient.blood_type = blood_type;
    patient.allergies = allergies;
    patient.insurance_id = insurance_id;

    // Save the updated patient 
    env.storage().instance().set(&DataKey::Patient(id), &patient);

    patient
    
   }





}

mod test;
