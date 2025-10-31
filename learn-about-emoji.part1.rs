fn main() {
    let rocket = "🚀";
    let smile = "😀";
    let family = "👨‍👩‍👧‍👦";
    
    println!("=== แสดง Emoji ปกติ ===");
    println!("Rocket: {}", rocket);
    println!("Smile: {}", smile);
    println!("Family: {}", family);
    
    println!("\n=== ข้อมูล Bytes ===");
    println!("Rocket bytes: {:?} (จำนวน: {} bytes)", rocket.as_bytes(), rocket.len());
    println!("Smile bytes: {:?} (จำนวน: {} bytes)", smile.as_bytes(), smile.len());
    println!("Family bytes: {:?} (จำนวน: {} bytes)", family.as_bytes(), family.len());
    
    // แสดง bytes แบบ hex
    println!("\n=== Hex Representation ===");
    print_hex("Rocket", rocket);
    print_hex("Smile", smile);
    print_hex("Family", family);
    
    // ทดลองแสดงแบบขาดช่วง (ทีละ byte)
    println!("\n=== แสดงแบบขาดช่วง (ทีละ byte) ===");
    print_partial("Rocket 🚀", rocket);
    print_partial("Smile 😀", smile);
    print_partial("Family 👨‍👩‍👧‍👦", family);
    
    // ทดลอง print โดยตรง (จะเห็น replacement character �)
    println!("\n=== Print bytes ไม่ครบโดยตรง ===");
    let bytes = rocket.as_bytes();
    for i in 1..=bytes.len() {
        print!("ใช้ {} byte(s): ", i);
        // แปลง bytes เป็น string (จะได้ � ถ้า invalid UTF-8)
        match std::str::from_utf8(&bytes[0..i]) {
            Ok(s) => println!("'{}' ✓ (valid UTF-8)", s),
            Err(_) => {
                // ใช้ String::from_utf8_lossy เพื่อแสดง � แทนที่ invalid bytes
                let s = String::from_utf8_lossy(&bytes[0..i]);
                println!("'{}' ✗ (invalid UTF-8)", s);
            }
        }
    }
}

fn print_hex(name: &str, s: &str) {
    print!("{}: ", name);
    for byte in s.as_bytes() {
        print!("{:02X} ", byte);
    }
    println!();
}

fn print_partial(name: &str, s: &str) {
    println!("\n{}:", name);
    let bytes = s.as_bytes();
    
    for i in 1..=bytes.len() {
        let partial = &bytes[0..i];
        let display = String::from_utf8_lossy(partial);
        
        print!("  {} byte(s): ", i);
        for b in partial {
            print!("{:02X} ", b);
        }
        print!("→ '{}'", display);
        
        // ตรวจสอบว่าเป็น valid UTF-8 หรือไม่
        if std::str::from_utf8(partial).is_ok() {
            println!(" ✓");
        } else {
            println!(" ✗ (incomplete/invalid)");
        }
    }
}