extern crate bitfield_rle;

#[test]
fn should_encode_decode() {
  let mut bits: Vec<u8> = vec![0; 16];
  bits[8] = 0b00000001;

  let enc = bitfield_rle::encode(&bits);
  assert_eq!(enc.len(), 4);

  let res = bitfield_rle::decode(enc).unwrap();

  assert_eq!(res[8], 0b00000001);
  assert_eq!(res, bits);
}

#[test]
fn encode_len() {
  let bitfield = vec![255, 255, 85, 84, 0, 0, 0, 183];
  let len = bitfield_rle::encode_len(bitfield);
  assert_eq!(len, 7);
}

#[test]
fn encode() {
  let bitfield = vec![255, 255, 85, 84, 0, 0, 0, 183];
  let enc = bitfield_rle::encode(&bitfield);
  let correct = vec![11, 4, 85, 84, 13, 2, 183];
  assert_eq!(enc.len(), correct.len());
  assert_eq!(enc, correct);
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
