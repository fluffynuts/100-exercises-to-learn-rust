struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  The following requirements should be met:
    //   - Only `To-Do`, `In Progress`, and `Done` statuses are allowed.
    //   - The `title` and `description` fields should not be empty.
    //   - the `title` should be at most 50 bytes long.
    //   - the `description` should be at most 500 bytes long.
    //  The method should panic if any of the requirements are not met.
    //
    // You'll have to use what you learned in the previous exercises,
    // as well as some `String` methods. Use the documentation of Rust's standard library
    // to find the most appropriate options -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(
        title: String, 
        description: String, 
        status: String,
    ) -> Self {
        let result = Self {
            title,
            description,
            status,
        };
        
        result.validate();
        return result;
    }
    
    fn validate(&self)  {
        match self.status.as_str() {
            s if s == "To-Do" => {},
            s if s == "In Progress" => {},
            s if s == "Done" => {},
            _ => panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed")
        }
        match self.title.len() {
            d if d == 0 => panic!("Title cannot be empty"),
            d if d > 50 => panic!("Title cannot be longer than 50 bytes"),
            _ => {}
        }
        match self.description.len() {
            0 => panic!("Description cannot be empty"),
            l if l > 500 => panic!("Description cannot be longer than 500 bytes"),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    use super::*;

    fn valid_description() -> String {
        return "Test Ticket Description".into();
    }

    fn valid_title() -> String {
        return "Test Ticket Title".into();
    }

    fn overly_long_title() -> String {
        return repeat("X").take(51).collect::<String>();
    }

    fn overly_long_description() -> String {
        return repeat('X').take(501).collect::<String>();
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
