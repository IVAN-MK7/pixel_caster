# pixel_caster

[![](https://img.shields.io/crates/v/pixel_caster.svg)](https://crates.io/crates/pixel_caster)

Get from, and send to, the screen RGBA values in bytes (8 bit unsigned integers, u8) to read or manipulate pixels.

## Examples

3 examples can be found in the examples directory. Use the following command, followed by the file title, to compile and run :

``` powershell
cargo run --example
```

## Get bytes

Example for "\examples\get_pixels_bytes.rs" :

``` powershell
cargo run --example get_pixels_bytes
```

The get_bytes function will return a Vec<u8> containing the bytes red from the pixels of a screen area of the requested size, starting from an absolute position on the screen.

<img src="media/example-get_pixels_bytes.png">


## Send bytes

Example for "\examples\send_pixel_bytes.rs" :

``` powershell
cargo run --example send_pixel_bytes
```

The send_bytes function will send a Vec<u8> containing the bytes to be applied the pixels of a screen area of the requested size, starting from an absolute position on the screen. In this example the Vec<u8> will contain 64 bytes representing a qube of 4 x 4 (16) pixels, where the first 2 will be red, the other 14 blue :

<img src="media/example-send_pixel_bytes.png">


## Clone pixels

Example for "\examples\clone_pixels_on_screen.rs" :

``` powershell
cargo run --example clone_pixels_on_screen
```

The copy_and_paste_pixels function will copy the pixels from a given area of the screen and paste them into another given area of the screen :

<img src="media/example-clone_pixels_on_screen.png">


Works only on Windows OS, starting from Windows 2000


## License

- [MIT license](http://opensource.org/licenses/MIT)