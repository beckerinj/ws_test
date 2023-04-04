use rand::Rng;

pub fn random_between_0_3() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..3)
}
