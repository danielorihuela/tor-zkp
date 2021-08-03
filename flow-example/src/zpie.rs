#[link(name = "zpie", kind = "static")]
extern "C" {
    pub fn init_setup();
    pub fn perform_setup();

    pub fn init_prover();
    pub fn generate_proof();

    pub fn init_verifier();
    pub fn verify_proof() -> u8;
}
