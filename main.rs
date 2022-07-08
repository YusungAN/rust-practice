use std::io;

fn main() {
    enum Condition {
        Sleepy,
        Tired,
        Studying(String),
        Playing
    }

    impl Condition {
        fn get_study_value(&self) {
            if let Condition::Studying(s) = self {
                println!("You're studying {}", s);
            }
        }
    }

    loop {
        let mut input = String::new();
        let mut input2 = String::new();
        println!("Set your condition:\n1. Sleepy\n2. Tired\n3. Studying\n4. Playing\ninput: ");

        io::stdin().read_line(&mut input).expect("Fail to read");

        let input: u32 = input.trim().parse().expect("Please type a number!");

        if input == 3 {
            println!("Please write what you are doing.");
            io::stdin().read_line(&mut input2).expect("Fail to read");
        }

        let input = match input {
            1 => Condition::Sleepy,
            2 => Condition::Tired,
            3 => Condition::Studying(input2),
            4 => Condition::Playing,
            _other => {
                println!("1 <= number <= 4");
                return;
            }
        };

        match input {
            Condition::Studying(_) => input.get_study_value(),
            _other => println!("Time is ticking.")
        }

        println!("If you don't want to change condition, press 'p', or type anything.");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Fail to read");

        if input3.chars().next() == Some('p') {
            break;
        }

    }
}
