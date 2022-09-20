#![no_main]

extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let data = std::io::Cursor::new(data);
    if let Ok(mut rdr) = lewton::inside_ogg::OggStreamReader::new(data) {
        let _ = rdr.stream_serial();
        let _ = rdr.get_last_absgp();
        let _ = rdr.read_dec_packet();
        let _ = rdr.read_dec_packet_itl();
    }
});
