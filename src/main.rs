use std::io;
use std::io::Write;
use std::process;

fn main() {
    loop {
        let verse_num: u32 = verse_input();
        // println!("verse_num = {verse_num}");
        print_verse(verse_num);
    }
}

fn verse_input() -> u32{
    // 프로그램 안내문 출력
    println!("본 프로그램은 크리스마스 캐롤  ‘The Twelve Days of Christmas’ 노래의 가사를 확인 할 수 있는 프로그램입니다.");

    loop{
        // 입력 안내물 출력
        print!("노래에서 알고싶은 절의 숫자 n(1~12)를 입력해주세요(종료하려면 0) : ");
        io::stdout().flush().expect("failed to flush stdout");

        // 입력
        let mut verse_num = String::new();
        io::stdin().read_line(&mut verse_num).expect("Failed to read line");


        // 유효성 검사
        // 1~12 사이의 자연수는 반환 후 함수 종료
        // 0이 나오면 프로그램 종료
        // 그외의 입력은 실패시키고 다시 안내문 출력으로 회귀
        match verse_num.trim().parse::<u32>() {
            Ok(natual_number) if natual_number == 0 => {
                println!("프로그램을 종료합니다.");
                process::exit(0);
            },
            Ok(natual_number) if (natual_number >= 1)&&(natual_number <= 12)=> return natual_number,
            _ => println!("유효하지 않은 입력입니다. 알고싶은 절의 숫자 n을 1과 12사이의 숫자로 입력해주세요."),
        }
    }
}

fn print_verse(verse_num: u32){
    // 입력 숫자에 따른 day 지정 및 공통 가사 출력
    let day_num = match verse_num{
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => unreachable!(), // 선택지 검증은 입력부에서 끝냈기 때문에 따로 만들지 않음.
    };
    println!("On the {day_num} day of Christmas,\nmy true love gave to me");

    // 입력된 verse_num~1 까지 -1로 감소하며 숫자에 맞는 가사 출력 반복
    for num in (1..=verse_num).rev(){
        match num{
            1 => println!("A partridge in a pear tree."),
            2 => print!("Two turtle doves,\nand "),
            3 => println!("Three French hens,"),
            4 => println!("Four calling birds,"),
            5 => println!("Five golden rings,"),
            6 => println!("Six geese a-laying,"),
            7 => println!("Seven swans a-swimming,"),
            8 => println!("Eight maids a-milking,"),
            9 => println!("Nine ladies dancing,"),
            10 => println!("Ten lords a-leaping,"),
            11 => println!("Eleven pipers piping,"),
            12 => println!("Twelve drummers drumming,"),
            _ => unreachable!(),
        }
    }
}