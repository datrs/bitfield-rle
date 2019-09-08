extern crate bitfield_rle;

#[test]
fn should_encode_decode() {
  let mut bits: Vec<u8> = vec![0; 16];
  bits[8] = 0b00000001;

  let enc = bitfield_rle::encode(&bits);
  assert_eq!(enc.len(), 6);

  let res = bitfield_rle::decode(enc).unwrap();

  assert_eq!(res[8], 0b00000001);
}

#[test]
fn decode_len() {
  let enc = [11, 4, 85, 84, 13, 2, 183];
  assert_eq!(8, bitfield_rle::decode_len(enc).unwrap());
}

#[test]
fn decode() {
  let enc = [11, 4, 85, 84, 13, 2, 183];
  let res = bitfield_rle::decode(enc).unwrap();
  let correct = vec![255, 255, 85, 84, 0, 0, 0, 183];
  assert_eq!(res, correct);
}
