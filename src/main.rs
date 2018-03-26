extern crate byteorder;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::net::UdpSocket;

// From page 18 of https://www.ietf.org/rfc/rfc5905.txt:
//
//  0                   1                   2                   3
//  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |LI | VN  |Mode |    Stratum     |     Poll      |  Precision   | 4B
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         Root Delay                            | 4B
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                         Root Dispersion                       | 4B
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                          Reference ID                         | 4B
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// +                     Reference Timestamp (64)                  + 8B
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// +                      Origin Timestamp (64)                    + 8B
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// +                      Receive Timestamp (64)                   + 8B
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ <- 40B
// |                                                               |
// +                      Transmit Timestamp (64)                  + 8B
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
const TRANSMIT_TS_OFFSET: u64 = 40;

// | LI | VN | Mode | .....
// where: LI = 0; VN, Mode = 3
// altogether: 000011011 -> 1b (base 16)
// padding the rest with zeros
const REQUEST_PACKET: &'static str = "\x1b\x00\x00\x00\x00\x00\x00\x00\
                                      \x00\x00\x00\x00\x00\x00\x00\x00\
                                      \x00\x00\x00\x00\x00\x00\x00\x00\
                                      \x00\x00\x00\x00\x00\x00\x00\x00\
                                      \x00\x00\x00\x00\x00\x00\x00\x00\
                                      \x00\x00\x00\x00\x00\x00\x00\x00";

// Unix epoch is 1970-01-01T00:00:00Z
// NTP epoch is 1900-01-01T00:00:00Z
// with 17 leap years in between
// (70 * 365 + 17) * 86400 = 2208988800
const UNIX_EPOCH_OFFSET: u32 = 2208988800;

const BIND_ADDRESS: &'static str = "0.0.0.0:0";

const BUFFER_SIZE: usize = 1024;

const NTP_PORT: u32 = 123;
const NTP_HOST: &'static str = "pool.ntp.org";

fn receive_timestamp() -> u32 {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let socket = UdpSocket::bind(BIND_ADDRESS).expect("couldn't bind to address");
    let ntp_address = format!("{}:{}", NTP_HOST, NTP_PORT);

    socket.send_to(REQUEST_PACKET.as_bytes(), ntp_address).expect("couldn't send data");
    socket.recv_from(&mut buffer).expect("didn't receive data");
    // TODO: handle timeout

    let mut reader = Cursor::new(&buffer);

    reader.set_position(TRANSMIT_TS_OFFSET);
    reader.read_u32::<BigEndian>().unwrap() - UNIX_EPOCH_OFFSET
    // TODO: extract milliseconds
}

fn main() {
    println!("{:?}", receive_timestamp());
}

#[test]
fn test() {
    use std::{thread, time};

    let t1 = receive_timestamp();
    thread::sleep(time::Duration::from_millis(1000));
    let t2 = receive_timestamp();

    assert!(t2 > t1);
    assert_eq!(t2 - t1, 1);
}
