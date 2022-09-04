//! # MRF Modbus Server
//!
//! ## Protocol
//!
//! [MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3](https://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf)
//!
//!
//! ## Data Model
//!
//! ### LEDs
//!
//! #### Register base address: xx
//!
//! | Registers   | Offset | Length | Type of    |
//! | ----------- | ------ | ------ | ---------- |
//! | Mode        | 0      | 1      | Read-Write |
//! | Color (RGB) | 1      | 2      | Read-Write |
//! | Lightness   | 3      | 1      | Read-Write |
//!
//!
//! #### Mode
//!
//! * 0 - Raw (Only color is valid)
//! * 1 - Color + Lightness
//! * 2 - Breath (Only color is valid)
//! * 3 - Rainbow (Color and lightness are invalid)
//!
//! Default: 2
//!
//!
//! #### Color (RGB)
//!
//! | Register0 |           | Register1 |           |
//! | --------- | --------- | --------- | --------- |
//! | Byte0     | Byte1     | Byte0     | Byte1     |
//! | RED       | GREEN     | BLUE      | 0x00      |
//!
//! Default: 0x00 0xFF 0x7F 0x00 (SpringGreen)
//!
//!
//! #### Lightness
//!
//! | Byte0 | Byte1     |
//! | ----- | --------- |
//! | 0x00  | 0x00~0xFF |
//!
//! Default: 0xFF

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use black_pill as _;
