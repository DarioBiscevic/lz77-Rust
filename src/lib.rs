pub mod lz_compress{
    pub fn lz77_compress(string: &String) -> Vec<u8>{

        let input = string.as_bytes();
        let mut output: Vec<u8> = Vec::new();

    
        let mut input_index = 0; //From where we read
        
        while input_index < input.len(){
    
            let mut offset = 0;
            let mut length: u8 = 0;
    
            let mut start_checking: i64 = (input_index as i64) - 255;
            if start_checking < 0{
                start_checking = 0;
            }


            let mut difference = input.len() - input_index;
    
            if difference > 255 {
                difference = 255;
            }

    
            let mut longest_index = 0;
            let mut longest_length = 0;
            
            for possible_index in (start_checking as usize)..input_index{
    
                let mut candidate_len = 0;
    
                if input[possible_index] == input[input_index]{
    
                    for i in 0..difference{
                        if input[possible_index + i] != input[input_index + i]
                        || possible_index + i >= input_index
                        {
                            break;
                        }else{
                            candidate_len += 1;
                        }
                    }
    
                    if candidate_len > longest_length{
                        longest_length = candidate_len;
                        longest_index = possible_index;
                    }
    
                }
    
            }
    
            if !(longest_length == 0 && longest_index == 0){
    
                offset = input_index - longest_index;
    
                for i in 0..difference{
                    if input[longest_index + i] != input[input_index + i]
                    || longest_index + i >= input_index
                    {
                        break;
                    }else{
                        length += 1;
                    }
                }
            }
    
    
    
            //println!("current_index: {} \t length: {}", current_index, length);
    
            output.push(offset as u8);
            output.push(length as u8);
    
            if input_index + (length as usize) < input.len(){
                output.push(input[input_index + (length as usize)]);
            }else{
                output.push(0);
                break;
            }
            
    
            input_index += length as usize;
    
            input_index += 1;
    
        }
    
        output
    }

    
    pub fn lz77_decompress(input: &Vec<u8>) -> String{

        use std::str;

        let mut output = String::new();
        let mut bytes: Vec<u8> = Vec::new();

        //UTF-8 encryption
        //0   - 127 => 1 byte
        //192 - 223 => 2 bytes
        //224 - 239 => 3 bytes
        //240 - 247 => 4 bytes

        let mut counter = 0;
        let mut current_index = 0;

        while counter < input.len(){

            let offset = input[counter] as usize;
            let length = input[counter + 1] as usize;
            let next_byte = input[counter + 2];

            for i in 0..length{
                bytes.push(bytes[current_index - offset + i]);
            }

            bytes.push(next_byte);

            //Length + next byte
            current_index += length + 1;

            counter += 3;

        }


        let mut counter = 0;

        while counter < bytes.len(){
            let mut bytecount = 0;

            if between(bytes[counter], 0, 128){
                bytecount = 1;
            }else if between(bytes[counter], 192, 224){
                bytecount = 2;
            }else if between(bytes[counter], 224, 240){
                bytecount = 3;
            }else if between(bytes[counter], 240, 247){
                bytecount = 4;
            }else{
                //TODO: throw error
            }

            let mut character: Vec<u8> = Vec::new();

            for i in 0..bytecount{
                character.push(bytes[counter + i]);
            }

            counter += bytecount;

            output.push_str(str::from_utf8(&character).unwrap());
        }

        output
    }


    fn between<T: std::cmp::PartialOrd>(val: T, a: T, b: T) -> bool{
        val >= a && val < b
    }
    
}