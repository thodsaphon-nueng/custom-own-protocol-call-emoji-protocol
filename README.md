# custom-own-protocol-call-emoji-protocol
repo  นี้เป็นการลองสร้าง custom protocol เพื่อแก้ปัญหาการส่ง emoji ที่เกิด cross protocol frame ได้


## repo structure
-  client 
-  server

การสร้าง repo / protocol เกิดจาก หากเรามาดูที่ emoji นั้นจะต้องประกอบ min 2 bytes และ ใช้ byte เยอะขึ้นได้ถึง 100 อันนี้ถาม chatgpt มาแต่ไม่แน่ใจอาจจะมากกว่านี้ครับ แสดงให้เห็นจาก code file learn-about-emoji.part1.rs


<img width="1818" height="532" alt="Image" src="https://github.com/user-attachments/assets/139948ea-c8a8-4bc6-8c73-e1946bbe0219" />

จะเห็นว่าแต่ละตัวใช้กี่ byte และ ความน่าสนใจคือ กรณีที่มันใหญ่มากๆ อาจจะทำให้ การแสดงผลผิด emoji ได้หากเรารับมาและ parse แบบไม่ครบ ตามรูปที่แคปมาให้ดู

<img width="1827" height="593" alt="Image" src="https://github.com/user-attachments/assets/9d669f74-9d1a-424e-924e-fbb0a141baa6" />

คำถามคือ เราสามารถสร้าง protocol ที่รองรับการส่ง emoji แบบยาวๆและรองรับกรณีที่ emoji มันใหญ่หว่า frame เราได้อย่างไร ผมจึงได้ลองสร้าง protocol นี้ขึ้นมาน่ะครับ

# Emoji Protocol
## 🧩 Text Frame Layout (Bit-level Overview)

```
┌──────────────┬──────────────┬──────────────────────────────┬────────────────────────────┬──────────────────────────────────────────────────────────┐
│ Byte Offset  │ Bit Range    │ Field Name                   │ Size (bits)                │ Description                                              │
├──────────────┼──────────────┼──────────────────────────────┼────────────────────────────┼──────────────────────────────────────────────────────────┤
│ 0            │ 7            │ FIN                          │ 1                         │ Final frame flag (1 = last frame, 0 = more frames)       │
│              │ 6            │ EMOJI_CROSS                  │ 1                         │ Emoji boundary flag (1 = emoji split, 0 = no split)      │
│              │ 5–0          │ RESERVED                     │ 6                         │ Reserved for future extensions                           │
├──────────────┼──────────────┼──────────────────────────────┼────────────────────────────┼──────────────────────────────────────────────────────────┤
│ 1            │ 7–0          │ CROSS_SIZE                   │ 8                         │ Emoji bytes in next frame (0–255)                        │
├──────────────┼──────────────┼──────────────────────────────┼────────────────────────────┼──────────────────────────────────────────────────────────┤
│ 2–3          │ 15–0         │ PAYLOAD LENGTH               │ 16 (Big-endian)            │ Payload size in bytes (0–4997)                           │
├──────────────┼──────────────┼──────────────────────────────┼────────────────────────────┼──────────────────────────────────────────────────────────┤
│ 4–503        │              │ PAYLOAD DATA                 │ 4000 (≈500 bytes)          │ Actual message data (must contain emoji)                 │
└──────────────┴──────────────┴──────────────────────────────┴────────────────────────────┴──────────────────────────────────────────────────────────┘
```

---

## 🧱 Header Breakdown (Binary Example)

```
Byte 0   |  Byte 1   |   Byte 2-3          |   Byte 4...
─────────┬────────────┬────────────────────┬──────────────────────────────
F E R----|----C-------|-------P------------|-------PAYLOAD DATA-----------
0 0 000000 00000000  00000000 00000000    <500 bytes of emoji data...>
─────────┴────────────┴────────────────────┴──────────────────────────────
↑ ↑ ↑           ↑               ↑
│ │ │           │               └── Payload length (16 bits, BE)
│ │ │           └── CROSS_SIZE (8 bits)
│ │ └── RESERVED (6 bits)
│ └── EMOJI_CROSS (1 bit)
└── FIN (1 bit)
```

---

## 📏 Frame Size Summary

| Component    | Size (bytes)  |
| ------------ | ------------- |
| Header       | 4             |
| Payload      | ≤500          |
| **MAX SIZE** | **504 bytes** |

📋 Field Descriptions</br>


FIN (1 bit): Final frame flag. 1 = last frame in message, 0 = more frames follow </br>
EMOJI_CROSS (1 bit): Emoji boundary crossing flag. 1 = emoji split across frames, 0 = no split  </br>
RESERVED (6 bits): Reserved for future protocol extensions  </br>
CROSS_SIZE (8 bits): Number of emoji bytes in next frame  </br>
PAYLOAD LENGTH (  16 bits ): Size of payload in bytes (0-4997). Big-endian byte order  </br>
PAYLOAD DATA ( 4000 bits ) : Actual message data. Must contain emoji  </br>


## การรันโปรแกรม

<img width="1900" height="958" alt="Image" src="https://github.com/user-attachments/assets/b7df2bda-26c4-4fd3-ae97-ca197809dd5f" />
<div align="center">
  Run server.
</div>
</br>


<img width="1916" height="970" alt="Image" src="https://github.com/user-attachments/assets/2fbc7d00-3f4a-4bb7-808d-e8e415ae0524" />
<div align="center">
  Run client part 1.
</div>
</br>

<img width="1912" height="961" alt="Image" src="https://github.com/user-attachments/assets/c7f243ee-bde0-4716-b679-4f66940480d2" />
<div align="center">
  Run client part 2.
</div>
</br>


<img width="1913" height="945" alt="Image" src="https://github.com/user-attachments/assets/446f99d4-dcaf-489d-9c27-34869b650097" />
<div align="center">
  Run client part 3.
</div>
</br>


<img width="1915" height="936" alt="Image" src="https://github.com/user-attachments/assets/c98716d4-c1ce-407a-ac4a-58addf9c1a36" />
<div align="center">
  server received part 1.
</div>
</br>


<img width="1902" height="957" alt="Image" src="https://github.com/user-attachments/assets/bddcb877-9832-4333-8b69-6fd3c8b914c4" />
<div align="center">
  server received part 2.
</div>
</br>

<img width="1900" height="961" alt="Image" src="https://github.com/user-attachments/assets/0e2b85e1-3463-4d9c-b4e4-0406457f6ec2" />
<div align="center">
  server received part 3.
</div>
</br>


> **_NOTE:_**  สามารถศึกษา emoji จาก files : learn-about-emoji.part1.rs , learn-about-emoji.part2.rs
