// ต้องเพิ่ม dependency นี้ใน Cargo.toml:
// [dependencies]
// unicode-segmentation = "1.10"

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let str = "🚀😀👨‍👩‍👧‍👦";
    
    println!("=== String ทั้งหมด ===");
    println!("String: {}", str);
    println!("Total bytes: {}", str.len());
    println!("Total chars: {}", str.chars().count());
    
    // วิธีที่ 1: ใช้ chars() - จะไม่ได้ผลดีกับ emoji ซับซ้อน
    println!("\n=== แบ่งด้วย chars() (ไม่ถูกต้องสำหรับ emoji ซับซ้อน) ===");
    for (i, ch) in str.chars().enumerate() {
        let char_str = ch.to_string();
        println!("Char {}: '{}' - {} byte(s)", i, char_str, char_str.len());
    }
    
    // วิธีที่ 2: ใช้ graphemes - วิธีที่ถูกต้อง! 🎯
    println!("\n=== แบ่งด้วย graphemes (ถูกต้อง!) ===");
    let graphemes: Vec<&str> = str.graphemes(true).collect();
    
    println!("จำนวน graphemes (emoji): {}", graphemes.len());
    
    for (i, grapheme) in graphemes.iter().enumerate() {
        println!("\nEmoji {}: '{}'", i + 1, grapheme);
        println!("  Bytes: {}", grapheme.len());
        println!("  Hex: {}", bytes_to_hex(grapheme.as_bytes()));
        println!("  Code points: {}", grapheme.chars().count());
    }
    
    // วิธีที่ 3: แสดงรายละเอียดแต่ละ emoji
    println!("\n=== รายละเอียดแต่ละ Emoji ===");
    analyze_emoji("🚀", "Rocket");
    analyze_emoji("😀", "Grinning Face");
    analyze_emoji("👨‍👩‍👧‍👦", "Family");
    
    // วิธีที่ 4: แบ่งและเก็บ byte ranges
    println!("\n=== Byte Ranges ของแต่ละ Emoji ===");
    print_grapheme_ranges(&str);
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn analyze_emoji(emoji: &str, name: &str) {
    println!("\n{}: '{}'", name, emoji);
    println!("  Total bytes: {}", emoji.len());
    println!("  Hex: {}", bytes_to_hex(emoji.as_bytes()));
    
    // แสดง code points
    let chars: Vec<char> = emoji.chars().collect();
    println!("  Code points: {}", chars.len());
    for (i, ch) in chars.iter().enumerate() {
        let cp = *ch as u32;
        println!("    {} U+{:04X} ({})", i + 1, cp, 
                 if cp == 0x200D { "ZWJ" } else { "Emoji" });
    }
}

fn print_grapheme_ranges(s: &str) {
    let mut byte_pos = 0;
    
    for (i, grapheme) in s.graphemes(true).enumerate() {
        let byte_len = grapheme.len();
        let end_pos = byte_pos + byte_len;
        
        println!("Emoji {}: '{}' → bytes [{}..{}] (length: {})", 
                 i + 1, 
                 grapheme, 
                 byte_pos, 
                 end_pos,
                 byte_len);
        
        byte_pos = end_pos;
    }
}

// ฟังก์ชันสำหรับตัดแบ่ง string ตาม grapheme ที่ปลอดภัย
fn split_at_grapheme(s: &str, grapheme_index: usize) -> Option<(&str, &str)> {
    let graphemes: Vec<&str> = s.graphemes(true).collect();
    
    if grapheme_index > graphemes.len() {
        return None;
    }
    
    let byte_index: usize = graphemes[..grapheme_index]
        .iter()
        .map(|g| g.len())
        .sum();
    
    Some(s.split_at(byte_index))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grapheme_split() {
        let str = "🚀😀👨‍👩‍👧‍👦";
        let graphemes: Vec<&str> = str.graphemes(true).collect();
        
        assert_eq!(graphemes.len(), 3);
        assert_eq!(graphemes[0], "🚀");
        assert_eq!(graphemes[1], "😀");
        assert_eq!(graphemes[2], "👨‍👩‍👧‍👦");
        
        assert_eq!(graphemes[0].len(), 4);  // 🚀 = 4 bytes
        assert_eq!(graphemes[1].len(), 4);  // 😀 = 4 bytes
        assert_eq!(graphemes[2].len(), 25); // 👨‍👩‍👧‍👦 = 25 bytes
    }
    
    #[test]
    fn test_safe_split() {
        let str = "🚀😀👨‍👩‍👧‍👦";
        
        let (left, right) = split_at_grapheme(str, 1).unwrap();
        assert_eq!(left, "🚀");
        assert_eq!(right, "😀👨‍👩‍👧‍👦");
        
        let (left, right) = split_at_grapheme(str, 2).unwrap();
        assert_eq!(left, "🚀😀");
        assert_eq!(right, "👨‍👩‍👧‍👦");
    }
}