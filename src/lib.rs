pub mod base64encoder {
    pub fn get_lookup_value(x: u8) -> char {
        assert!(x <= 63);
        return match x {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            8 => 'I',
            9 => 'J',
            10 => 'K',
            11 => 'L',
            12 => 'M',
            13 => 'N',
            14 => 'O',
            15 => 'P',
            16 => 'Q',
            17 => 'R',
            18 => 'S',
            19 => 'T',
            20 => 'U',
            21 => 'V',
            22 => 'W',
            23 => 'X',
            24 => 'Y',
            25 => 'Z',
            26 => 'a',
            27 => 'b',
            28 => 'c',
            29 => 'd',
            30 => 'e',
            31 => 'f',
            32 => 'g',
            33 => 'h',
            34 => 'j',
            35 => 'j',
            36 => 'k',
            37 => 'l',
            38 => 'm',
            39 => 'n',
            40 => 'o',
            41 => 'p',
            42 => 'q',
            43 => 'r',
            44 => 's',
            45 => 't',
            46 => 'u',
            47 => 'v',
            48 => 'w',
            49 => 'x',
            50 => 'y',
            51 => 'z',
            52 => '0',
            53 => '1',
            54 => '2',
            55 => '3',
            56 => '4',
            57 => '5',
            58 => '6',
            59 => '7',
            60 => '8',
            61 => '9',
            62 => '+',
            63 => '/',
            64_u8..=u8::MAX => panic!("Invalid value to convert."),
        };
    }

    pub fn is_padding_necessary(bytes: &Vec<u8>) -> bool {
        if bytes.len() % 3 == 0 {
            return false;
        }

        return true;
    }

    pub fn encode(values: &Vec<u8>) -> String {
        let padding_necessary = is_padding_necessary(values);

        let mut working_value: u8 = 0;
        let mut bits_in_working_value = 0;

        let mut j: usize = 0;
        let mut sextets_output: i32 = 0;

        let mut encoded_string = String::new();

        for _i in 0..values.len() {
            if j % 3 == 0 {
                let sextet = values[j] >> 2;

                // Just grab the last two bits that were cut off
                working_value = values[j] & 0x3;
                bits_in_working_value = 2;

                encoded_string.push(get_lookup_value(sextet));

                sextets_output += 1;
            }

            if j % 3 == 1 {
                // working_value at this point should contain the last-two bits of the
                // previously analyzed byte.

                // So we need the first four bits
                let mut sextet = values[j] >> 4;
                working_value = working_value << 4;
                sextet |= working_value;

                // Set the working value to the lower 4 bits of the byte
                working_value = values[j] & 0xF;
                bits_in_working_value = 4;

                encoded_string.push(get_lookup_value(sextet));
                sextets_output += 1;
            }

            if j % 3 == 2 {
                // Just need the first two bits this time
                let mut sextet = values[j] >> 6;

                working_value = working_value << 2;
                sextet |= working_value;

                encoded_string.push(get_lookup_value(sextet));

                // Need the last 6 bits of the current byte to complete the last value
                // before repeating the triad.
                sextet = values[j] & 0x3F;
                encoded_string.push(get_lookup_value(sextet));

                // Reset the working value
                working_value = 0;
                bits_in_working_value = 0;

                sextets_output += 2;
            }

            j += 1;

            // We have now processed the last
            if j > values.len() - 1 {
                break;
            }
        }

        // Pad result if necessary
        if padding_necessary {
            let mut padding_sextets_required = (-sextets_output).rem_euclid(4);
            let positions_to_shift = 6 - bits_in_working_value;

            encoded_string.push(get_lookup_value(working_value << positions_to_shift));

            padding_sextets_required -= 1;

            for _i in 0..padding_sextets_required {
                encoded_string.push('=');
            }
        }

        return encoded_string;
    }
}
