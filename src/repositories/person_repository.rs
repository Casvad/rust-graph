use std::fmt::format;
use crate::models::person::Person;

trait PersonRepository {
    fn save_person(&self, person: Person) -> Person;
    fn find_by_id(&self, person_id: String) -> Option<Person>;
    fn delete_person(&self, person_id: String) -> Option<Person>;
    fn find_all(&self) -> Vec<Person>;
}

struct PersonRepositoryImpl {
    file_name: String,
}

impl PersonRepository for PersonRepositoryImpl {

    fn save_person(&self, person: Person) -> Person {
        let person_string = serde_json::to_string(&person).expect("Cannot parse person");
        let line_format = format!("{}\n", person_string);
        match std::fs::write(self.file_name.to_string(), line_format){
            Ok(()) => {
                println!("Success saving user {}", person.uuid)
            }
            Err(e) => {
                println!("Error saving user because {}", e.to_string())
            }
        };

        person
    }

    fn find_by_id(&self, person_id: String) -> Option<Person> {
        todo!()
    }

    fn delete_person(&self, person_id: String) -> Option<Person> {
        todo!()
    }

    fn find_all(&self) -> Vec<Person> {
        todo!()
    }
}