use mines;

fn main() {

    loop {
        let s = mines::get_state();

        println!("{}", s);

        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();

        let mut iter = command.split(" ");

        let x = iter.next().unwrap().trim().parse::<usize>().unwrap();
        let y = iter.next().unwrap().trim().parse::<usize>().unwrap();

        mines::open_field(x, y);
    }
}
