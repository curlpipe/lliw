use lliw::{Fg, Bg, Style, Reset};

fn main() {
    // Prints "Blue" in a blue colour
    println!("{}Blue{}", Fg::Blue, Fg::Reset);
    // Prints "Bold" in bold
    println!("{}Bold{}", Style::Bold, Style::NoBold);
    // Prints "Green" with a green background
    println!("{}Green{}", Bg::Green, Bg::Reset);
    
    // You can even use it in more complicated ways
    println!(
        "{}{}Attention!{}{} You have {}{}1{}{} new message",
        Style::Underline, Fg::Yellow,
        Style::NoUnderline, Fg::Reset,
        Bg::White, Fg::Black,
        Bg::Reset, Fg::Reset,
    );
    
    // You can make them go over the top of each other too
    println!(
        "{}Hello{} Wor{}ld! My{} Name{} {}Is{} Lliw{}",
        Style::Italic, Fg::LightPurple, 
        Bg::Black, Fg::Reset, Style::NoItalic, 
        Style::Underline, Bg::Reset, Reset
    );

    // Don't like these long formatting macros? You can use it like this too!
    print!("{}", Fg::LightRed);
    print!("Hello\nThere!");
    print!("{}\n", Reset);

    // You can also use an RGB value if you want 24-bit colours
    println!(
        "{}R{}a{}i{}n{}b{}o{}w{}",
        Fg::Rgb(255, 0, 0),
        Fg::Rgb(255, 128, 0),
        Fg::Rgb(255, 255, 0),
        Fg::Rgb(0, 255, 0),
        Fg::Rgb(0, 255, 255),
        Fg::Rgb(128, 0, 255),
        Fg::Rgb(255, 0, 128),
        Fg::Reset,
    );
}
