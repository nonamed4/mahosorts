mod easy_input {
    pub fn clear_screen() {
        print!("\x1Bc");
    }

    pub fn input() -> String {
        let mut user_input = String::new();

        match std::io::stdin().read_line(&mut user_input) {
            Ok(_) => {}
            Err(_) => {
                println!("Warning: Unexpected error while taking input.");
                return input();
            }
        }


        String::from(user_input.trim())
    }

    pub fn string_to_i32(value: String) -> Result<i32, String> {
        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err(String::from("Input isn't a number!")),
        };
        Ok(value)
    }


}

mod mahosort_on_four {
    /// Sorts the slice in groups of two.
    /// Warning: It doesn't completely sort it.
    pub fn sort_on_twos<T: Ord + Copy>(slice: &mut [T]) {
        for i in 0..slice.len() / 2 {
            if slice[2 * i] > slice[2 * i + 1] {
                (slice[2 * i], slice[2 * i + 1]) = (slice[2 * i + 1], slice[2 * i]);
            }
        }
    }

    /// Sorts the slice in groups of four.
    /// Warning: It doesn't completely sort it.
    pub fn sort_on_fours<T: Ord + Copy>(slice: &mut [T]) {
        sort_on_twos(slice);
        let (mut min1, mut min2, mut max1, mut max2): (T, T, T, T);

        for i in 0..slice.len() / 4 {
            // Finding min1 and max1.
            if slice[4 * i] > slice[4 * i + 2] {
                min1 = slice[4 * i + 2];
                max1 = slice[4 * i];
            } else {
                max1 = slice[4 * i + 2];
                min1 = slice[4 * i];
            }

            // Finding min2 and max2.
            if slice[4 * i + 1] > slice[4 * i + 3] {
                min2 = slice[4 * i + 3];
                max2 = slice[4 * i + 1];
            } else {
                max2 = slice[4 * i + 3];
                min2 = slice[4 * i + 1];
            }

            slice[4 * i] = min1;
            slice[4 * i + 3] = max2;

            if max1 > min2 {
                slice[4 * i + 1] = min2;
                slice[4 * i + 2] = max1;
            } else {
                slice[4 * i + 1] = max1;
                slice[4 * i + 2] = min2;
            }
        }
    }

    /// Sorts the slice with an extra Vec<T> in an O(n log n) time complexity algorithm.
    pub fn mahosort_on_four<T: Ord + Copy>(vector: &mut Vec<T>) {
        sort_on_fours(vector);

        // Extra vector ): (Causes O(n) space complexity.)
        let mut dummy: Vec<T> = Vec::with_capacity(vector.len());

        for i in 0..vector.len() / 4 {
            four_merge(&mut dummy, [vector[4 * i], vector[4 * i + 1], vector[4 * i + 2], vector[4 * i + 3]]);
        }
        // This loop will work only if sorting is completed (slice.len() mod 4 != 0)
        for i in dummy.len()..vector.len() {
            dummy.insert(dummy.binary_search(&vector[i]).unwrap_or_else(|x| x), vector[i]);
        }

        *vector = dummy;
    }

    pub fn four_merge<T: Ord + Copy>(vector: &mut Vec<T>, values: [T; 4]) {
        let mut rl: usize = vector.binary_search(&values[0]).unwrap_or_else(|x| x);
        vector.insert(rl, values[0]);
        rl += 1;
        let mut rr_cons: usize = vector.binary_search(&values[3]).unwrap_or_else(|x| x);
        vector.insert(rr_cons, values[3]);

        for val in vec![values[1], values[2]] {
            let mut rr = rr_cons;
            while rr != rl {
                if val > vector[(rl + rr) / 2] {
                    if rr == rl + 1 {
                        rl += 1;
                        break;
                    } else {
                        rl = (rl + rr) / 2;
                    }
                } else {
                    if rr == rl + 1 {
                        // rr -= 1;
                        break;
                    } else {
                        rr = (rl + rr)/2;
                    }
                }
            }
            vector.insert(rl, val);
            rr_cons += 1;
            rl += 1;
        }
    }
}
