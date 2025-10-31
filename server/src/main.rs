use std::net::{TcpListener, TcpStream};
use std::io::{self,  Read, Write, Result, Error, ErrorKind};

use unicode_segmentation::UnicodeSegmentation;
//### PROTOCOL

const MAX_FRAME_SIZE: usize = 504;
const HEADER_SIZE: usize = 4;
const MAX_PAYLOAD_SIZE: usize = MAX_FRAME_SIZE - HEADER_SIZE; // 500

// Header flags
const FIN_BIT: u8 = 0b10000000;
const EMOJI_CROSS_BIT: u8 = 0b01000000;

#[derive(Debug, Clone)]
pub struct Frame {
    pub fin: bool,
    pub emoji_cross: bool,
    pub cross_size: u8, // จำนวน bytes ของ emoji ที่เหลือใน frame ถัดไป
    pub payload: Vec<u8>,
}

impl Frame {

    pub fn new(payload: Vec<u8>, fin: bool) -> Self {
        Self {
            fin,
            emoji_cross: false,
            cross_size: 0,
            payload,
        }
    }


    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(HEADER_SIZE + self.payload.len());
        
        // Byte 0: flags
        let mut flags = 0u8;
        if self.fin {
            flags |= FIN_BIT; // bit7 = FIN
        }
        if self.emoji_cross {
            flags |= EMOJI_CROSS_BIT; // bit6 = EMOJI_CROSS
        }
        // bit5..0 = RESERVED (6 bits = 0)
        buffer.push(flags);

        // ---------- Byte 1: CROSS_SIZE ----------
        buffer.push(self.cross_size as u8); // 0–255

        // ---------- Bytes 2–3: PAYLOAD LENGTH (big-endian) ----------
        let len = self.payload.len() as u16;
        buffer.push((len >> 8) as u8);      // high byte
        buffer.push((len & 0xFF) as u8);    // low byte

        // ---------- Payload ----------
        buffer.extend_from_slice(&self.payload);

        buffer

    }

}
//### END PROTOCOL



pub struct ProtocolReader<R: Read> {
    reader: R,
}

impl<R: Read> ProtocolReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }



    pub fn receive_message(&mut self) -> Result<Vec<Vec<u8>>> {
        let mut messages = Vec::new();
        let mut message = Vec::new();
        

        let mut remain_emoji = 0;
        
        loop {
    // อ่าน header
    let mut header = [0u8; HEADER_SIZE];
    match self.reader.read_exact(&mut header) {
        Ok(_) => {},
        Err(e) => {
            if e.kind() == ErrorKind::UnexpectedEof {
                return Err(Error::new(ErrorKind::ConnectionReset, "Connection closed by peer"));
            }
            return Err(e);
        }
    }
    
    println!("{:?}", header);
    let flags = header[0];
    let cross_emoji_size = header[1];
    let payload_len = ((header[2] as u16) << 8) | (header[3] as u16);
    
    let mut payload = vec![0u8; payload_len as usize];
    self.reader.read_exact(&mut payload)?;
    println!("p {:?}", payload);
    
    // ตรวจสอบ FIN_BIT ก่อน
    if (flags & FIN_BIT) != 0 {
        message.extend_from_slice(&payload);
        if !message.is_empty() {
            messages.push(message.clone());
        }
        break;
    }
    
    // จัดการกรณีที่กำลังรอ emoji ที่ขาดอยู่
    if remain_emoji != 0 {
        let take = remain_emoji.min(payload.len());
        message.extend_from_slice(&payload[0..take]);
        remain_emoji -= take;
        
        // ถ้า emoji ครบแล้ว
        if remain_emoji == 0 {
            messages.push(message.clone());
            println!("v. {}", take);
            message = Vec::new();
            
            // ข้อมูลที่เหลือเริ่ม message ใหม่
            if take < payload.len() {
                message.extend_from_slice(&payload[take..]);
                // ตั้ง remain_emoji ใหม่ถ้ามี cross emoji
                if cross_emoji_size != 0 {
                    remain_emoji = cross_emoji_size as usize;
                }
            }
        }
    } else {
        // ไม่ได้รอ emoji - เก็บ payload
        message.extend_from_slice(&payload);
        
        // ถ้ามี cross emoji ให้ตั้ง remain_emoji
        if cross_emoji_size != 0 {
            remain_emoji = cross_emoji_size as usize;
        } else {
            // ไม่มี cross emoji - payload สมบูรณ์
            messages.push(message.clone());
            message = Vec::new();
        }
    }
}

println!("MES, {:?}, {}", messages, messages.len());
Ok(messages)
    }



}

pub fn run_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("🚀 Server listening on {}", addr);
    println!("📡 Waiting for connections...\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_client(stream: TcpStream) {
    let peer = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => return,
    };
    
    println!("📥 Client connected: {}", peer);


    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to clone stream: {}", e);
            return;
        }
    };

    let mut reader = ProtocolReader::new(reader_stream);

    loop {
        match reader.receive_message() {
            Ok(messages) => {
                
                // println!("l {:?}", messages[1]);
                
               for (i, chunk) in messages.iter().enumerate() {
    match std::str::from_utf8(chunk) {
        Ok(s) => {
            // นับ grapheme clusters (emoji ที่มนุษย์เห็น)
            let grapheme_count = s.graphemes(true).count();
            println!("Chunk {}: {} graphemes", i, grapheme_count);
            
            // แสดงแต่ละ grapheme
            for (j, g) in s.graphemes(true).enumerate() {
                println!("  [{}] {} ({} bytes)", j, g, g.len());
                
            }
        },
        Err(e) => {
            println!("Chunk {}: invalid UTF-8 at byte {}", i, e.valid_up_to());
        }
    }
    println!();
}

                println!("✅ Echoed back to {}", peer);
            }
            Err(e) => {
                match e.kind() {
                    ErrorKind::ConnectionReset | ErrorKind::UnexpectedEof => {
                        println!("👋 Client disconnected: {}", peer);
                    }
                    _ => {
                        eprintln!("❌ Error reading from {}: {}", peer, e);
                    }
                }
                break;
            }
        }
    }

}

fn main() {
    let addr = "127.0.0.1:8080";
    run_server(addr).unwrap();
}
