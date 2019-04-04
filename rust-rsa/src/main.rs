fn main() {

    // 64 bit key
    let key: String = "suWoe6Ii".to_string();
    let kbytes: Vec<u8> = key.into_bytes();   

    // 56 bit
    let p_1 = vec![57u8, 49, 41, 33, 25, 17, 9,
                        1, 58, 50, 42, 34, 26, 18,
                        10, 2, 59, 51, 43, 35, 27,
                        19, 11, 3, 60, 52, 44, 36,
                        63, 55, 47, 39, 31, 23, 15,
                        7, 62, 54, 46, 38, 30, 22,
                        14, 6, 61, 53, 45, 37, 29,
                        21, 13, 5, 28, 20, 12, 4];
    // 48 bit
    let _p_2 = vec![14u8, 17, 11, 24, 1, 5,
                        3, 28, 15, 6, 21, 10,
                        23, 19, 12, 4, 26, 8,
                        16, 7, 27, 20, 13, 2,
                        41, 52, 31, 37, 47, 55,
                        30, 40, 51, 45, 33, 48,
                        44, 49, 39, 56, 34, 53,
                        46, 42, 50, 36, 29, 32];

    let permed: Vec<u8> = permute(&p_1, &kbytes); 
   
    print!("input: ");
    print_bits(&kbytes);
    print!("output: ");
    print_bits(&permed);

}

fn permute(perm: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {

    let mut output: Vec<u8> = Vec::new();
    // prepare new byte for output
    let mut new_byte: u8 = 0b0000_0000; 
    // byte number in input 
    let mut byte_num: u8;
    // bit number in byte of input
    let mut bit_num: u8;

    for i in 0..perm.len() {

        // get byte num of input and bit num of byte
        byte_num = 0;
        if perm[i] >= 8 {
            byte_num = perm[i] / 8; 
        }
        bit_num = perm[i] % 8;

        // shift left 1 bit
        new_byte <<= 1;
        
        // check if bit is 1
        if input[byte_num as usize] & (1 << bit_num) != 0 {
            new_byte ^= 0b0000_0001;
        }

        // filled up our byte, on to the next
        if (i+1) % 8 == 0 {
            output.push(new_byte);  
            new_byte = 0b0000_0000;
        }
    }
    return output;
}

fn vec8_to_bitstring(input: &Vec<u8>) -> String {

    let mut output: String = String::new();
    for byte_num in 0..input.len() {
        for bit_num in (0..8).rev() {
            if input[byte_num as usize] & (1 << bit_num) != 0 {
                output.push('1');                
            }
            else{
                output.push('0');
            }
        }
    }
    return output;
}

fn bitstring_to_vec8(input: &String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut byte: u8 = 0b0000_0000;

    for (i,c) in input.chars().enumerate() {
       
       byte <<= 1; 
       if c == '1' {
         byte ^= 0b0000_0001;
       }

       if (i +1) % 8 == 0 {
           bytes.push(byte);
           byte = 0b0000_0000;
       }
    }
    return bytes;
}

fn print_bits(input: &Vec<u8>) {
    for i in 0..input.len() {
        print!("{:08b}", input[i]);
    }
    println!("\n");
}
