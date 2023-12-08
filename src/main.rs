use std::env;

struct Size {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
}

impl Size {
    fn new(bytes: u64, kilobytes: u64, megabytes: u64, gigabytes: u64) -> Self {
        Size {
            bytes,
            kilobytes,
            megabytes,
            gigabytes,
        }
    }
}

// implement Debug custom trait for Size struct
impl std::fmt::Debug for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sizes")
            .field("bytes", &self.bytes)
            .field("kilobytes", &self.kilobytes)
            .field("megabytes", &self.megabytes)
            .field("gigabytes", &self.gigabytes)
            .finish()
    }
}

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

impl FileSize {
    fn new(unit: &str, qty: u64) -> Result<Self, String> {
        let file_size = match unit.to_lowercase().as_str() {
            "bytes" => FileSize::Bytes(qty),
            "kb" => FileSize::Kilobytes(qty),
            "mb" => FileSize::Megabytes(qty),
            "gb" => FileSize::Gigabytes(qty),
            _ => return Err("enter correct unit in input text".to_string()),
        };
        Ok(file_size)
    }
}

fn extract_qty(split_text: &[&str]) -> Result<u64, String> {
    let qty = if let Some(quantity) = split_text.get(0) {
        println!("quantity: {:}", quantity);
        if let Ok(quantity) = quantity.parse::<u64>() {
            quantity
        } else {
            return Err("enter correct quantity  as number in input text".to_string());
        }
    } else {
        return Err("enter correct quantity in input text; missing qquantity".to_string());
    };
    Ok(qty)
}

fn extract_unit(split_text: &[&str]) -> Result<String, String> {
    let ut = if let Some(unit) = split_text.get(1) {
        match unit.to_lowercase().as_str() {
            "bytes" | "kb" | "mb" | "gb" => unit.to_string(),
            _ => return Err("enter correct unit in input text".to_string()),
        }
    } else {
        return Err("enter correct unit in input text; missing unit".to_string());
    };
    Ok(ut)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if let Some(text) = args.get(1) {
        println!("text: {}", text);
        let split_text = text.split_whitespace().collect::<Vec<&str>>();
        println!("split_text: {:?}", split_text);
        let qty = extract_qty(&split_text).expect("Failed to extract quantity");
        println!("qty: {}", qty);
        let unit = extract_unit(&split_text).expect("Failed to extract unit");
        println!("unit: {}", unit);
        let file_size = FileSize::new(unit.as_str(), qty).expect("Failed to create FileSize");
        println!("Actual fileSize: {:?}", file_size);
        let size = match file_size {
            FileSize::Bytes(bytes) => {
                println!("{} bytes", bytes);
                Size::new(bytes, bytes / 1000, bytes / 1000_000, bytes / 1000_000_000)
            }
            FileSize::Kilobytes(kb) => {
                println!("{:.2} KB", kb as f64);
                Size::new(kb * 1000, kb, kb / 1000, kb / 1000_000)
            }
            FileSize::Megabytes(mb) => {
                println!("{:.2} MB", mb as f64);
                Size::new(mb * 1000_000, mb * 1000, mb, mb / 1000)
            }
            FileSize::Gigabytes(gb) => {
                println!("{:.2} GB", gb as f64);
                Size::new(gb * 1000_000_000, gb * 1000_000, gb * 1000, gb)
            }
        };
        // debug size struct with all fields for sizes
        println!("{:?} ", size);
    } else {
        println!("No argument passed");
    }
}
