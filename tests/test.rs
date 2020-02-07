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

#[test]
fn not_power_of_two() {
  let deflated = bitfield_rle::encode(vec![255, 255, 255, 240]);
  let inflated = bitfield_rle::decode(deflated).unwrap();
  assert_eq!(inflated, vec![255, 255, 255, 240]);
}

#[test]
/// Differs on NodeJS: node trims final bits when 0 and returns a smaller payload
/// Decoding returns the same result, but encoding the result is smaller
/// Both are interoperable, with the different on the payload size when reading from node.
///
/// ```js
/// require('bitfield-rle').encode(Buffer.from([])) // => <Buffer >
/// require('bitfield-rle').decode(Buffer.from([])) // => <Buffer >
/// require('bitfield-rle').decode(Buffer.from([0])) // => <Buffer >
/// ```
fn encodes_empty_bitfield() {
  assert_eq!(
    bitfield_rle::decode(bitfield_rle::encode(vec![])).unwrap(),
    vec![]
  );
  assert_eq!(bitfield_rle::decode(vec![]).unwrap(), vec![]);
  assert_eq!(bitfield_rle::decode(vec![0]).unwrap(), vec![]);
  assert_eq!(bitfield_rle::encode(vec![]), vec![0]);
}

#[test]
/// Differs on NodeJS: node trims final bits when 0 and returns a smaller payload
/// Decoding returns the same result, but encoding the result is smaller.
/// Both are interoperable, with the different on the payload size when reading from node.
///
/// ```js
/// var data = require('bitfield-rle').decode(Buffer.from([2, 64, 253, 31])) // => <Buffer 40 00...>
/// var data = require('bitfield-rle').encode(data) // => <Buffer 02 40>
/// var data = require('bitfield-rle').encode(data) // => <Buffer 40> skipping the last bits
/// ```
fn does_not_trims_remaining_bytes() {
  let mut bitfield = vec![0; 1024];
  bitfield[0] = 64;
  assert_eq!(bitfield_rle::encode_len(&bitfield), 4);
  assert_eq!(bitfield_rle::encode(&bitfield), vec![2, 64, 253, 31]);
}
