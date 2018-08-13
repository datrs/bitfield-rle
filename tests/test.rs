extern crate bitfield_rle;

#[test]
fn should_encode_decode() {
  let mut bits: Vec<u8> = vec![0; 16];
  bits[8] = 0b00000001;

  let enc = bitfield_rle::encode(&bits);
  assert_eq!(enc.len(), 6);

  let (res, _len) = bitfield_rle::decode(enc).unwrap();

  assert_eq!(res[8], 0b00000001);
}
