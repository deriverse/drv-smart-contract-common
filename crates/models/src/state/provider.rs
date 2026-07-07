// use bytemuck::{Pod, Zeroable};

// pub struct Provider {
//     pub accounts_length: usize,
//     pub data_length: usize,
//     pub amount_in_offset: usize,
//     pub destination_token_acc_index: usize,
// }

// pub const PROVIDERS: [Provider; 5] = [
//     Provider {
//         // Orca
//         destination_token_acc_index: 2,
//         accounts_length: 11,
//         data_length: 56,
//         amount_in_offset: 8,
//     },
//     Provider {
//         // Orca v2
//         destination_token_acc_index: 6,
//         accounts_length: 15,
//         data_length: 72,
//         amount_in_offset: 8,
//     },
//     Provider {
//         // Raydium AMM
//         destination_token_acc_index: 16,
//         accounts_length: 17,
//         data_length: 40,
//         amount_in_offset: 1,
//     },
//     Provider {
//         // Raydium CLMM
//         destination_token_acc_index: 6,
//         accounts_length: 13,
//         data_length: 41,
//         amount_in_offset: 8,
//     },
//     Provider {
//         // Raydium CPMM
//         destination_token_acc_index: 5,
//         accounts_length: 13,
//         data_length: 24,
//         amount_in_offset: 8,
//     },
// ];

// #[cfg(test)]
// pub mod test {
//     use bytemuck::{bytes_of_mut, Pod, Zeroable};

//     use crate::state::provider::Provider;

//     #[repr(C)]
//     #[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
//     pub struct SwapDataA {
//         pub tag: u8,
//         pub padding: u8,
//         pub field_a: u16,
//         pub field_b: u32,
//         pub amount_in: u64,
//     }

//     pub const PROVIDER_A: Provider = Provider {
//         accounts_length: 0,
//         data_length: std::mem::size_of::<SwapDataA>(),
//         amount_in_offset: std::mem::size_of::<u8>()
//             + std::mem::size_of::<u8>()
//             + std::mem::size_of::<u16>()
//             + std::mem::size_of::<u32>(),
//         destination_token_acc_index: 0,
//     };

//     #[test]
//     pub fn test_data_construction() {
//         let mut swap_data_a = SwapDataA::default();

//         let data = bytes_of_mut(&mut swap_data_a);

//         let amount_in: u64 = 10;

//         data[PROVIDER_A.amount_in_offset..PROVIDER_A.amount_in_offset + std::mem::size_of::<u64>()]
//             .copy_from_slice(&amount_in.to_le_bytes());

//         let swap_data_a: &SwapDataA = bytemuck::from_bytes(data);

//         assert_eq!(swap_data_a.amount_in, amount_in);
//         assert_eq!(swap_data_a.field_b, 0);
//         assert_eq!(swap_data_a.field_a, 0);
//         assert_eq!(swap_data_a.tag, 0);
//     }
// }
