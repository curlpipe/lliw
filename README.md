# Lliw 
*Roughly pronounced khlew*

Lliw provides colours for your terminal, with no additional dependencies.

# Aims

- No dependencies
- Provides colours and styles in a non-opinionated way
- Provides multiple ways to use
- Doesn't make your code look like trash
- Doesn't assume how you'll use the crate

# Installation

If you have `cargo-edit` installed, it's as easy as:

```sh
cargo add lliw
```

if you don't have `cargo-edit` you can add the following to your `Cargo.toml` file

```toml
[dependencies]
lliw = "0"
```

# Example usage

```rust
use lliw::{Fg, Bg, Style};

fn main() {
	// Prints "Blue" in a blue colour
	println!("{}Blue{}", Fg::Blue, Fg::Reset);
	// Prints "Bold" in bold
	println!("{}Bold{}", Style::Bold, Style::NoBold);
	// Prints "Green" with a green background
	println!("{}Green{}", Bg::Green, Bg::Reset);
	
	// You can even use it in more complicated ways
	println!(
		"{}{}Attention!{}{} You have {}1{} new message",
		Style::Underline, Fg::Blue,
		Style::NoUnderline, Fg::Reset,
		Bg::Green, Bg::Reset,
	);
}
```

# Usage

There are 3 enums provided with lliw, `Fg`, `Bg` and `Style`.
There is also 1 struct, `Reset`.

- `Fg` - Control the text colour
	+ Consists of the types: Black, Red, Green, Yellow, Blue, Purple, Cyan, White,
    LightBlack, LightRed, LightGreen, LightYellow,
    LightBlue, LightPurple, LightCyan, LightWhite and Reset.
    + The colours are reset using the `Reset` variant.
- `Bg` - Control the text background colour
	+ Consists of the types: Black, Red, Green, Yellow, Blue, Purple, Cyan, White,
    LightBlack, LightRed, LightGreen, LightYellow,
    LightBlue, LightPurple, LightCyan, LightWhite and Reset.
    + The colours are reset using the `Reset` variant.
- `Style` - Control the text styles
	+ Consists of the types: Bold, NoBold, Underline, NoUnderline, Strike, NoStrike, Italic, NoItalic, Inverse, NoInverse, Faint and NoFaint.
		+ Bold: Make the text bold, can be terminated with `NoBold`
		+ Underline: Make the text have an underline, can be terminated with `NoUnderline`
		+ Italic: Make the text have an underline, can be terminated with `NoUnderline`
		+ Inverse: Make the text have an underline, can be terminated with `NoUnderline`
		+ Faint: Make the text have an underline, can be terminated with `NoUnderline`
- `Reset` - This is a full reset struct that resets foreground, background and style when used.

Be sure to check out the docs over at https://docs.rs/lliw

License: MIT
