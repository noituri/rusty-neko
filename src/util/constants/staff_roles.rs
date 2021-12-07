use std::collections::HashMap;

pub fn get_staff_roles() -> HashMap<String, u64> {
    let mut map: HashMap<String, u64> = HashMap::new();

    map.insert("Lead Staff".to_owned(), 578903414562357288);

    map.insert("Moderator".to_owned(), 566364651986747392);

    map.insert("Support".to_owned(), 568155071997542410);

    map
}
