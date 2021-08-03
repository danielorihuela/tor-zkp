#!/bin/bash

circom circuit/main.circom --r1cs --wasm --sym
snarkjs r1cs info main.r1cs
snarkjs r1cs print main.r1cs main.sym
