use std::fs;
use std::panic::set_hook;
use anyhow::{anyhow, Result};

use crate::models::{DBState, Epic, Status, Story};

pub struct JiraDatabase {
    pub(crate) database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }
    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
        let last_id = parsed.last_item_id;
        let new_id =last_id +1;
        parsed.last_item_id = new_id;
        parsed.epics.insert(new_id, epic);
        self.database.write_db(&parsed)?;
        Ok(new_id)

    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut  parsed = self.database.read_db()?;
        let last_id = parsed.last_item_id;
        let new_id = last_id + 1;

        parsed.last_item_id =new_id;
        parsed.stories.insert(new_id, story);
        parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("could not fnd epic in database"))?.stories.push(new_id);
        self.database.write_db(&parsed)?;
        Ok(new_id)

    }
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        for stroy_id in &parsed.epics.get(&epic_id).ok_or_else(||anyhow!("could not find epic in datbase!"))?.stories {
           parsed.stories.remove(stroy_id);
        }
        parsed.epics.remove(&epic_id);
        self.database.write_db(&parsed)?;
        Ok(())
    }
    pub fn delete_story(&self, epic_id: u32,story_id: u32) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        let epic = parsed.epics.get_mut(&epic_id).ok_or_else(||anyhow!("could not find epic in database!"))?;
        let strory_index = epic.stories.iter().position(|id| id == &story_id).ok_or_else(||anyhow!("story id not found in epic stories vector"))?;
        epic.stories.remove(strory_index);
        parsed.stories.remove(&story_id);
        self.database.write_db(&parsed)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        parsed.epics.get_mut(&epic_id).ok_or_else(||anyhow!("could not find epic in database!"))?.status = status;
        self.database.write_db(&parsed)?;
        Ok(())
    }
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        let mut parsed = self.database.read_db()?;
        parsed.epics.get_mut(&story_id).ok_or_else(||anyhow!("could not find story in database!"))?.status = status;
        self.database.write_db(&parsed)?;
        Ok(())
    }
}
 pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let parsed: DBState = serde_json::from_str(&db_content)?;
        Ok(parsed)
    }

   /*
    fs::write(&amp;self.file_path, &amp;serde_json::to_vec(db_state)?)? is the main operation of
    this method. It writes the serialized byte vector to a file.
    The path to the file is specified by self.file_path, which is assumed to be a field
    in the struct that contains this method.
    The fs::write function writes the data to the file at the specified path.
    The ? operator is again used to handle any errors that might occur during the file write operation
    */
    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}

impl JSONFileDatabase {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

pub mod test_utils {
    use super::*;
    use crate::models::DBState;
    use std::{cell::RefCell, collections::HashMap};
    /*
        The code snippet `las_wrten_state: RefCell<DBState>` is a declaration in Rust. It defines a mutable wrapper around a `DBState` object using `RefCell`, which allows for interior mutability. This means that you can mutate the data inside the `RefCell` even if the `RefCell` itself is immutable. Here's a step-by-step explanation:
    1. **`las_wrten_state`:** This is the name of the variable being declared. It seems to be a typo and might have been intended to be `last_written_state` or something similar. This variable will hold a `RefCell` that wraps around a `DBState` object.
    2. **`RefCell`:** This is a type provided by the Rust standard library, specifically from the `std::cell` module. `RefCell` is used to achieve interior mutability, meaning that it allows you to borrow and mutate the value it wraps at runtime, even if the `RefCell` itself is not mutable. This is particularly useful when you need to mutate data but are constrained by Rust's borrowing rules.
    3. **`<DBState>`:** This indicates that `RefCell` is a generic type that wraps a value of type `DBState`. `DBState` is presumably a struct or another type defined elsewhere in the code, representing some state related to a database (as suggested by its name).
    ### Step-by-Step Explanation:
    - **Step 1:** The code declares a variable named `las_wrten_state`. This variable is of type `RefCell<DBState>`.
    - **Step 2:** The `RefCell` is used to wrap an instance of `DBState`. This wrapping allows the `DBState` instance to be mutated through the `RefCell`, even if `las_wrten_state` itself is immutable.
    - **Step 3:** The `RefCell` provides two main methods for accessing and modifying the underlying data:
      - `borrow()`: This method is used to create an immutable reference to the data inside the `RefCell`. It checks at runtime to ensure that there are no mutable references active.
      - `borrow_mut()`: This method is used to create a mutable reference to the data inside the `RefCell`. It checks at runtime to ensure that there are no other references (mutable or immutable) active.
    - **Step 4:** By using `RefCell`, you can safely mutate the `DBState` while adhering to Rust's borrowing rules, as long as you manage the borrow checks at runtime.
    In summary, the code snippet is setting up a mechanism to safely and flexibly mutate a `DBState` instance within Rust's strict ownership and borrowing system.
         */
    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }
    /*
    Box: In Rust, Box is a smart pointer that allocates memory on the heap. It allows
    you to store data that has a size unknown at compile time.
    dyn Database: The dyn keyword is used to create a trait object, which allows for dynamic
    dispatch. dyn Database means any type that implements the Database trait.
    Box: This combination means a heap-allocated pointer to any type that implements the Database trait, enabling polymorphic behavior and dynamic dispatch. This is useful when the exact type of the database isn't known until runtime.
         */
    impl MockDB {
        pub fn new() -> Self {
            Self {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::MockDB;
    use super::*;

    #[test]
    fn create_epic_should_work() {
        let db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let result = db.create_epic(epic.clone());
        assert_eq!(result.is_ok(), true);
        let id = result.unwrap();
        let db_state = db.read_db().unwrap();

        let expected_id =1;
        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id(){
        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let story = Story::new("".to_owned(), "".to_owned());
        let non_existent_epic_id = 999;

        let result = db.create_story(story, non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id(){
        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let non_existent_epic_id = 999;
        let result = db.delete_epic(non_existent_epic_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_epic_should_work(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let epic = Epic::new("".to_owned() ,"".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_epic(epic_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id =2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None );
        assert_eq!(db_state.stories.get(&epic_id), None );



        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};

        let epic = Epic::new("".to_owned() ,"".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id =result.unwrap();
        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id =result.unwrap();
        let  non_existent_epic_id = 999;
        let result = db.delete_story(non_existent_epic_id,story_id);
        assert_eq!(result.is_err(), true);

    }
    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let epic = Epic::new("".to_owned() ,"".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());


        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id =result.unwrap();

        let result = db.create_story(story, epic_id);
        assert_eq!(result.is_ok() ,true);

        let non_existent_story_id = 999;

        let result = db.delete_story(epic_id, non_existent_story_id);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn delete_story_should_work(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let epic = Epic::new("".to_owned() ,"".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.create_story(story,epic_id);
        assert_eq!(result.is_ok(), true);

        let story_id = result.unwrap();

        let result = db.delete_story(epic_id, story_id);
        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id).unwrap().stories.contains(&story_id), false);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invlid_epic_id(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let non_existent_epic_id = 999;

        let reuslt = db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert_eq!(reuslt.is_err(), true);
    }

    #[test]
    fn update_epic_status_should_work(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = db.create_epic(epic);
        assert_eq!(result.is_ok(), true);

        let epic_id = result.unwrap();

        let result = db.update_epic_status(epic_id, Status::Closed);

        assert_eq!(result.is_ok(), true);

        let db_state = db.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);


    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};

        let non_existent_story_id = 999;

        let result = db.update_story_status(non_existent_story_id, Status::Closed);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn update_story_status_should_work(){

        let db = JiraDatabase {database:Box::new(MockDB::new())};
    }
    mod database {
        use std::collections::HashMap;
        use std::io::Write;
        use std::ptr::write;
        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path(){
            let db = JSONFileDatabase {file_path: "INVALID_PATH".to_owned()};
            assert_eq!(db.read_db().is_err(), true);
        }
        #[test]
        fn read_db_should_fail_with_invalid_json(){
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{"last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile.path().to_str().expect("failed to convert tmpfile path to str").to_string()
            };
            let result =  db.read_db();
            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file(){
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{"last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile.path().to_str().expect("failed to convert tmpfile path to str").to_string()
            };
            let result =  db.read_db();
            assert_eq!(result.is_ok(), true);
        }
        #[test]
        fn write_db_should_work(){
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
            let file_contents = r#"{"last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile.path().to_str().expect("failed to convert tmpfile path to str").to_string()
            };
            let story = Story {name: "epic1".to_owned(), description: "epic1".to_owned(), status: Status::Closed};
            let epic= Epic {name: "epic1".to_owned(), description: "epic1".to_owned(), status: Status::Closed, stories: vec![2]};
            let mut stories =HashMap::new();
            stories.insert(2, story);
            let mut epics = HashMap::new();
            epics.insert(2,epic);
            let state = DBState{last_item_id: 2, epics, stories};

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);

        }
    }
}
