extern crate bitfield_rle as rle;
extern crate sparse_bitfield as bitfield;

use bitfield::Bitfield;

#[test]
fn should_encode_decode () {
  let mut bits = Bitfield::new(1024);
  bits.set(400, true);

  let len = bits.byte_len();
  let mut bytes = Vec::with_capacity(len);
  bits.to_bytes(bytes);

  let len = rle::encode_len(bytes);
  let enc = Vec::with_capacity(len);
  rle::encode(bytes, enc);
  assert_eq(enc.len(), 6);

  let len = rle::decode_len(enc);
  let dec = Vec::with_capacity(len);
  rle::decode(enc, dec);

  let bits = Bitfield::from_bytes(dec);
  assert_eq(bits.get(400), true);
}
