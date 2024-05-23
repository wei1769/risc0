// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use json_core::Outputs;
use json_methods::SEARCH_JSON_ELF;
use risc0_zkvm::{
    stark_to_snark,
    get_prover_server, recursion::identity_p254, CompactReceipt, ExecutorEnv, ProverOpts, Receipt,
};
fn main() {
    let data = include_str!("../res/example.json");
    let outputs = search_json(data);
    println!();
    println!("  {:?}", outputs.hash);
    println!(
        "provably contains a field 'critical_data' with value {}",
        outputs.data
    );
}

fn search_json(data: &str) -> Outputs {
    let env = ExecutorEnv::builder()
        .write(&data)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let opts = ProverOpts::default();
    let prover = get_prover_server(&opts).unwrap();
    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, SEARCH_JSON_ELF).unwrap().receipt;
    // let receipt = prover.prove(env, SEARCH_JSON_ELF).unwrap();
    // let claim = receipt.get_claim().unwrap();
    
    let succinct_receipt = prover.compsite_to_succinct(&receipt.inner.composite().unwrap()).unwrap();

    let ident_receipt = identity_p254(&succinct_receipt).unwrap();
    let seal_bytes = ident_receipt.get_seal_bytes();

    let seal = stark_to_snark(&seal_bytes).unwrap().to_vec();
    let claim = receipt.claim().unwrap();
    let receipt = Receipt::new(
        risc0_zkvm::InnerReceipt::Compact(CompactReceipt { seal, claim }),
        receipt.journal.bytes,
    );
    receipt.journal.decode().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let data = include_str!("../res/example.json");
        let outputs = super::search_json(data);
        assert_eq!(
            outputs.data, 47,
            "Did not find the expected value in the critical_data field"
        );
    }
}
