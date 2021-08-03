#!/bin/bash

wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_13.ptau
snarkjs zkey new main.r1cs powersOfTau28_hez_final_13.ptau main_0000.zkey
snarkjs zkey contribute main_0000.zkey main_final.zkey
snarkjs zkey export verificationkey main_final.zkey verification_key.json
