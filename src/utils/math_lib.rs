use rand::Rng;

pub fn get_rand_num() -> i32 {
    let mut rng = rand::thread_rng();
    let rand_val = rng.gen_range(64..74);
    return rand_val;
}