#[derive(Clone, Copy)]
enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Empty,
}

struct Board{
    values: Vec<Vec<Value>>,
}

impl Board{
    fn display(&self){
        print!("|");
        for _ in 0..9{
            print!("---");
        }
        println!("--------|");
        
        for i in 0..9{
            for j in 0..9{
                print!("|");
                print!("{}", show_value(self.get_value(j, i)));
            }

            print!("|\n|");
            for _ in 0..9{
                print!("---");
            }
            println!("--------|");
        }
    }
    
    fn get_value(&self, x: usize, y: usize) -> Value{
        self.values[y][x]
    }

}

fn show_value(thing: Value) -> String{
    match thing {
        Value::One => String::from(" 1 "),
        Value::Two => String::from(" 2 "),
        Value::Three => String::from(" 3 "),
        Value::Four => String::from(" 4 "),
        Value::Five => String::from(" 5 "),
        Value::Six  => String::from(" 6 "),
        Value::Seven => String::from(" 7 "),
        Value::Eight  => String::from(" 8 "),
        Value::Nine  => String::from(" 9 "),
        Value::Empty => String::from("   "),
    }
}

fn default_board() -> Board{
    Board{
        values: vec![
            vec![Value::Empty, Value::Empty, Value::Six, Value::Empty, Value::Empty, Value::Empty, Value::Five, Value::Empty, Value::Eight],
            vec![Value::One, Value::Empty, Value::Two, Value::Three, Value::Eight, Value::Empty, Value::Empty, Value::Empty, Value::Four],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Two, Value::Empty, Value::Empty, Value::One, Value::Nine, Value::Empty],
            vec![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Six, Value::Three, Value::Empty, Value::Four, Value::Five],
            vec![Value::Empty, Value::Six, Value::Three, Value::Four, Value::Empty, Value::Five, Value::Eight, Value::Seven, Value::Empty],
            vec![Value::Five, Value::Four, Value::Empty, Value::Nine, Value::Two, Value::Empty, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Empty, Value::Eight, Value::Seven, Value::Empty, Value::Empty, Value::Four, Value::Empty, Value::Empty, Value::Empty],
            vec![Value::Two, Value::Empty, Value::Empty, Value::Empty, Value::Nine, Value::Eight, Value::Four, Value::Empty, Value::Seven],
            vec![Value::Four, Value::Empty, Value::Nine, Value::Empty, Value::Empty, Value::Empty, Value::Three, Value::Empty, Value::Empty],

        ],
    }
}

fn main() {
    let a = default_board();
    a.display();
}
