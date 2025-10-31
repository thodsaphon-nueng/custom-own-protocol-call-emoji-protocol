use std::io::{Read, Write, Result, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
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


pub fn split_into_frames(data: &str) -> Vec<Frame> {

    let mut frames = Vec::new();
   

    let mut emoji_cross = false;
    let mut cross_size = 0u8;

    let graphemes: Vec<&str> = data.graphemes(true).collect();
    let data = data.as_bytes();
    

    // 0-500 byte 1 frame
    if MAX_PAYLOAD_SIZE >=  data.len() {
        let chunk = data.to_vec();
        let is_final = true;

        let mut frame = Frame::new(chunk, is_final);
        frame.emoji_cross = emoji_cross;
        frame.cross_size = cross_size;
        
        frames.push(frame);
    }else{


        let mut offset = 0;
let mut chunk: Vec<u8> = Vec::new();
let mut chunk_size_remain = MAX_PAYLOAD_SIZE;

for grapheme in graphemes.iter() {
    let gbytes = grapheme.as_bytes();

    if gbytes.len() <= chunk_size_remain {
        chunk.extend_from_slice(gbytes);
        chunk_size_remain -= gbytes.len();
    } else {
        // ต้องตัด frame ที่นี่
        let (first_part, second_part) = gbytes.split_at(chunk_size_remain);

        // frame ปัจจุบัน
        let mut frame = Frame::new(
            {
                let mut f = chunk.clone();
                f.extend_from_slice(first_part);
                f
            },
            false
        );
        frame.emoji_cross = true;
        frame.cross_size = second_part.len() as u8;
        frames.push(frame);

        // เตรียม chunk ใหม่จากส่วนที่เหลือ
        chunk.clear();
        chunk.extend_from_slice(second_part);
        chunk_size_remain = MAX_PAYLOAD_SIZE - chunk.len();
    }
}

// สุดท้าย
if !chunk.is_empty() {
    let mut frame = Frame::new(chunk, true);
    frame.emoji_cross = false;
    frame.cross_size = 0;
    frames.push(frame);
}



    }

    frames


}

pub struct ProtocolWriter<W: Write> {
    writer: W,
}
impl<W: Write> ProtocolWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn send_message(&mut self, data: &str) -> Result<()> {
        let frames = split_into_frames(data);
        
        for frame in frames {

            println!("{:?}", frame);

            let encoded = frame.encode();

            // for (i, byte) in encoded.iter().enumerate() {
            //     println!("Byte {}: {:08b}", i, byte);
            // }

            self.writer.write_all(&encoded)?;
            self.writer.flush()?;
        }
        
        Ok(())
    }
}


pub fn run_client(addr: &str, message: &str) -> Result<()> {


    let stream = TcpStream::connect(addr)?;
    println!("✅ Connected to {}", addr);

    let mut writer = ProtocolWriter::new(stream);

    // ส่งข้อความ
    println!("📤 Sending: {}", message);
    println!("");

    let graphemes: Vec<&str> = message.graphemes(true).collect();
    for (i, grapheme) in graphemes.iter().enumerate() {
        println!("{}", grapheme);
    }




    writer.send_message(message)?;

    Ok(())
}


fn main() {
    // println!("Hello, world!");


    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage:");
        println!("  Client: cargo run -- client [addr] [message]");
        println!("\nExamples:");
        println!("  cargo run -- client 127.0.0.1:8080 \"🚀😀👨‍👩‍👧‍👦\"");
        return;
    }


    match args[1].as_str() {
        "client" => {
            let addr = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:8080");
            
            //test case 1

            // let message = args.get(3).map(|s| s.as_str())
            //     .unwrap_or("🚀😀👨‍👩‍👧‍👦");

            //test case 2

            // let emoji1 = "👨‍👩‍👧‍👦";
            // let emoji2 = "😀";
            // let repeated = emoji1.repeat(19)+&emoji2.repeat(7);
            // let message = args.get(3).map(|s| s.as_str())
            //     .unwrap_or(&repeated);

            //test case 3
            let emoji1 = "👨‍👩‍👧‍👦";
            let emoji2 = "😀";
            let emoji3 = "👩‍❤️‍💋‍👨";
            let repeated = emoji1.repeat(19)+&emoji2.repeat(7)+&emoji3.repeat(20);
            let message = args.get(3).map(|s| s.as_str())
                .unwrap_or(&repeated);

            run_client(addr, message).unwrap();
        }
        _ => println!("Invalid command. Use 'server' or 'client'"),
    }
}
