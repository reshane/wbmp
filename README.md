# WBMP encoding and decoding library

WBMP encoding and decoding library written in Rust.

### Decoding

```rust
use std::fs::File;
use std::io::BufReader;
use wbmp::decoder::WbmpDecoder;

let f = BufReader::new(File::open("path/to/file.wbmp").unwrap());
let mut decoder = WbmpDecoder::new(f)?;
let (width, height) = decoder.dimensions();

let mut img_data = vec![0_u8; (width * height) as usize];
decoder.read_image_data(img_data.as_mut_slice())?;
```

### Encoding

```rust
use std::fs::File;
use wbmp::encoder::WbmpEncoder;

let img_data = vec![
    0xFF, 0x00, 0xFF, 0x00,
    0xFF, 0x00, 0xFF, 0x00,
    0xFF, 0x00, 0xFF, 0x00,
    0xFF, 0x00, 0xFF, 0x00,
];
let (width, height) = (4, 4);
let mut wbmpep = File::create("checker.wbmp")?;
let mut encoder = WbmpEncoder::new(&mut wbmpep);
encoder.encode(&img_data, width, height, wbmp::color::ColorType::Luma8)?;
```

