include "eddsaposeidon.circom"
include "bitify.circom"

template sign() {
    signal input a;
    signal input b;
    signal output result;

    component difference_sign = Num2Bits(8);
    difference_sign.in <== a - b;
    result <== difference_sign.out[7];
}

template verify_reputation() {
    signal output result;

    signal input enabled;
    signal input Ax;
    signal input Ay;
    signal input texp;
    signal input tnow;
    signal input tlength;

    signal private input R8x;
    signal private input R8y;
    signal private input S;
    signal private input reputation;
    signal private input message;

    signal reputation_and_texp_match_message <-- ~~(message - texp - (reputation * 10 ** tlength));
    component eqCheckX = ForceEqualIfEnabled();
    eqCheckX.enabled <== enabled;
    eqCheckX.in[0] <== reputation_and_texp_match_message;
    eqCheckX.in[1] <== 0;

    component sign_timestamp = sign();
    sign_timestamp.a <== texp;
    sign_timestamp.b <== tnow;
    signal not_expired <== sign_timestamp.result;
    not_expired === 0;

    component sign_rep = sign();
    sign_rep.a <== reputation;
    sign_rep.b <== 0;
    result <== sign_rep.result;
    result === 0;

    component eddsa = EdDSAPoseidonVerifier();
    eddsa.enabled <== enabled;
    eddsa.Ax <== Ax;
    eddsa.Ay <== Ay;
    eddsa.S <== S;
    eddsa.R8x <== R8x;
    eddsa.R8y <== R8y;
    eddsa.M <== message;
}

component main = verify_reputation();
