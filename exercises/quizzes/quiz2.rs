// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

use my_module::transformer;

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }

    fn uppercase(input: String) -> String {
        input.to_uppercase()
    }

    fn trim(input: String) -> String {
        input.trim().to_string()
    }

    fn append(input: String, count: usize) -> String {
        // let mut result: String = input.clone();
        // let concat: &str = "bar";
        
        // for _ in 1..count {
        //     result = result.clone() + concat;
        // }

        let result = format!("{}{}", input, "bar".repeat(count));

        result
    }

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result = Vec::new();

        for (text, command) in input {
            match command {
                Command::Uppercase => result.push(uppercase(text)),
                Command::Trim => result.push(trim(text)),
                Command::Append(int) => result.push(append(text, int)),
            }
        }

        result
    }
}

fn main() {
    // You can optionally experiment here.

    let input = vec![
        ("test".to_string(), Command::Uppercase),
        (" test ".to_string(), Command::Trim),
        ("test".to_string(), Command::Append(1)),
    ];
    let output = transformer(input);
    println!("{:?}", output);
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
