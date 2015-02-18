fn hex_to_decimal(hex: u8) -> u8 {
  if '0' as u8 <= hex && hex <= '9' as u8 {
    hex - '0' as u8
  } else if hex >= 'A' as u8 && hex <= 'Z' as u8 {
    hex - 'A' as u8 + 10
  } else {
    hex - 'a' as u8 + 10
  }
}

fn decimal_to_base64(dec: u8) -> u8 {
  match dec {
    n @ 0...25 => ('A' as u8) + n,
    n @ 26...51 => ('a' as u8) + (n - 26),
    n @ 52...61 => ('0' as u8) + (n - 52),
    62 => '+' as u8,
    63 => '/' as u8,
    n @ _ => panic!("Shouldn't happen: {}", n)
  }
}

fn hex_string_to_binary(hex_string: &str) -> Vec<u8> {
  let mut code_point: u8 = 0;
  let mut pair = false;
  let mut code_points: Vec<u8> = Vec::new();

  for ch in hex_string.chars() {
    let num = hex_to_decimal(ch as u8);

    if pair {
      code_points.push(code_point | num);
      code_point = 0;
      pair = false;
    } else {
      code_point = num << 4;
      pair = true;
    }
  }

  code_points
}

fn binary_to_base64(chars: Vec<u8>) -> String {
  let mut output: String = String::new();

  let total_6_bit_chars = chars.len() * 8 / 6;

  for ch in 0..total_6_bit_chars {
    let start_bit = ch * 6;
    let start_byte = start_bit / 8;

    let mut result: u16 = (chars[start_byte] as u16) << 8;

    if start_byte + 1 < chars.len() {
      result |= chars[start_byte + 1] as u16;
    }

    let shift = 16 - ((start_bit % 8) + 6);
    let base_64_dec = ((result >> shift) & 0x3F) as u8;
    let base_64_char = decimal_to_base64(base_64_dec) as char;

    output.push(base_64_char);
  }

  output
}

// fn vec_xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
//   a.iter().zip(b).collect().map(|c,d| c ^ d)
// }

fn s1c1() {
  let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let chars = binary_to_base64(hex_string_to_binary(input));

  println!("{}", chars);
}

// fn s1c2() {
//   let a = "1c0111001f010100061a024b53535009181c";
//   let b = "686974207468652062756c6c277320657965";
//   let binary_a = hex_string_to_binary(a);
//   let binary_b = hex_string_to_binary(b);

//   let result = vec_xor(binary_a, binary_b);

//   println!("{}", binary_to_base64(result));
// }

fn main() {
  s1c1();
}
