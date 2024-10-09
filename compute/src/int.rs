use crate::uint::GarbledUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::convert::From;
use std::marker::PhantomData;
use tandem::states::{Contributor, Evaluator};
use tandem::Circuit;

pub type GarbledInt1 = GarbledInt<1>;
pub type GarbledInt2 = GarbledInt<2>;
pub type GarbledInt4 = GarbledInt<4>;
pub type GarbledInt8 = GarbledInt<8>;
pub type GarbledInt16 = GarbledInt<16>;
pub type GarbledInt32 = GarbledInt<32>;
pub type GarbledInt64 = GarbledInt<64>;
pub type GarbledInt128 = GarbledInt<128>;

// Define a new type GarbledInt<N>
#[derive(Debug, Clone)]
pub struct GarbledInt<const N: usize> {
    pub(crate) bits: Vec<bool>, // Store the bits of the signed integer (in two's complement)
    _phantom: PhantomData<[bool; N]>, // PhantomData to ensure the N bit size
}

// Implement GarbledInt<N>
impl<const N: usize> GarbledInt<N> {
    // Constructor for GarbledInt<N> from a boolean vector
    pub fn new(bits: Vec<bool>) -> Self {
        assert_eq!(bits.len(), N, "The number of bits must be {}", N);
        GarbledInt {
            bits,
            _phantom: PhantomData,
        }
    }

    /// Simulates the local execution of the circuit using a 2 Party MPC protocol.
    ///
    /// The Multi-Party Computation is performed using the full cryptographic protocol exposed by the
    /// [`Contributor`] and [`Evaluator`]. The messages between contributor and evaluator are exchanged
    /// using local message queues. This function thus simulates an MPC execution on a local machine
    /// under ideal network conditions, without any latency or bandwidth restrictions.
    pub fn simulate(
        &self,
        circuit: &Circuit,
        input_contributor: &[bool],
        input_evaluator: &[bool],
    ) -> anyhow::Result<Vec<bool>> {
        let mut eval = Evaluator::new(
            circuit.clone(),
            input_evaluator,
            ChaCha20Rng::from_entropy(),
        )?;
        let (mut contrib, mut msg_for_eval) =
            Contributor::new(circuit, input_contributor, ChaCha20Rng::from_entropy())?;

        tracing::debug!("contributor ciphertext: {:?}", hex::encode(&msg_for_eval));

        assert_eq!(contrib.steps(), eval.steps());

        for _ in 0..eval.steps() {
            let (next_state, msg_for_contrib) = eval.run(&msg_for_eval)?;
            eval = next_state;

            let (next_state, reply) = contrib.run(&msg_for_contrib)?;
            contrib = next_state;

            msg_for_eval = reply;
        }
        Ok(eval.output(&msg_for_eval)?)
    }
}

impl<const N: usize> From<GarbledUint<N>> for GarbledInt<N> {
    fn from(uint: GarbledUint<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed Int<N>
        GarbledInt {
            bits: uint.bits,
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<&GarbledUint<N>> for GarbledInt<N> {
    fn from(uint: &GarbledUint<N>) -> Self {
        // Directly copy the bits from the unsigned Uint<N> to the signed Int<N>
        GarbledInt {
            bits: uint.bits.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<const N: usize> From<i8> for GarbledInt<N> {
    fn from(value: i8) -> Self {
        assert!(N <= 8, "Int<N> can only support up to 8 bits for i8");

        // Convert i8 to bits, least-significant bit first (two's complement)
        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i16> for GarbledInt<N> {
    fn from(value: i16) -> Self {
        assert!(N <= 16, "Int<N> can only support up to 16 bits for i16");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i32> for GarbledInt<N> {
    fn from(value: i32) -> Self {
        assert!(N <= 32, "Int<N> can only support up to 32 bits for i32");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i64> for GarbledInt<N> {
    fn from(value: i64) -> Self {
        assert!(N <= 64, "Int<N> can only support up to 64 bits for i64");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<i128> for GarbledInt<N> {
    fn from(value: i128) -> Self {
        assert!(N <= 128, "Int<N> can only support up to 128 bits for i128");

        let mut bits = Vec::with_capacity(N);
        let mut mask = 1;

        for _ in 0..N {
            bits.push((value & mask) != 0);
            mask <<= 1;
        }

        GarbledInt::new(bits)
    }
}

impl<const N: usize> From<GarbledInt<N>> for i8 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 8, "Int<N> can only be converted to i8 if N <= 8");

        let mut value: i8 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i16 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 16, "Int<N> can only be converted to i16 if N <= 16");

        let mut value: i16 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i32 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 32, "Int<N> can only be converted to i32 if N <= 32");

        let mut value: i32 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i64 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 64, "Int<N> can only be converted to i64 if N <= 64");

        let mut value: i64 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

impl<const N: usize> From<GarbledInt<N>> for i128 {
    fn from(gint: GarbledInt<N>) -> Self {
        assert!(N <= 128, "Int<N> can only be converted to i128 if N <= 128");

        let mut value: i128 = 0;
        for (i, &bit) in gint.bits.iter().enumerate() {
            if bit {
                value |= 1 << i;
            }
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::{GarbledUint128, GarbledUint16, GarbledUint32, GarbledUint64, GarbledUint8};

    #[test]
    fn test_from_negative_i8() {
        let a: GarbledInt8 = (-2_i8).into(); // Two's complement binary for -2 is 11111110
        let result: i8 = a.into();
        assert_eq!(result, -2_i8);
    }

    #[test]
    fn test_from_positive_i8() {
        let a: GarbledInt8 = 3_i8.into(); // Binary for 3 is 00000011
        let result: i8 = a.into();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_from_negative_i16() {
        let a: GarbledInt16 = (-21845_i16).into(); // Two's complement binary for -21845 is 1010101010101011
        let result: i16 = a.into();
        assert_eq!(result, -21845);
    }

    #[test]
    fn test_from_positive_i16() {
        let a: GarbledInt16 = 21845_i16.into(); // Binary for 21845 is 0101010101010101
        let result: i16 = a.into();
        assert_eq!(result, 21845);
    }

    #[test]
    fn test_from_negative_i32() {
        let a: GarbledInt32 = (-1431655765_i32).into(); // Two's complement binary for -1431655765 is 10101010101010101010101010101011
        let result: i32 = a.into();
        assert_eq!(result, -1431655765);
    }

    #[test]
    fn test_from_positive_i32() {
        let a: GarbledInt32 = 1431655765_i32.into(); // Binary for 1431655765 is 01010101010101010101010101010101
        let result: i32 = a.into();
        assert_eq!(result, 1431655765);
    }

    #[test]
    fn test_from_negative_i64() {
        let a: GarbledInt64 = (-6148914691236517205_i64).into(); // Two's complement binary for -6148914691236517205 is 1010101010101010101010101010101010101010101010101010101010101011
        let result: i64 = a.into();
        assert_eq!(result, -6148914691236517205);
    }

    #[test]
    fn test_from_positive_i64() {
        let a: GarbledInt64 = 6148914691236517205_i64.into(); // Binary for 6148914691236517205 is 0101010101010101010101010101010101010101010101010101010101010101
        let result: i64 = a.into();
        assert_eq!(result, 6148914691236517205);
    }

    #[test]
    fn test_from_negative_i128() {
        let a: GarbledInt128 = (-6148914691236517205_i128).into(); // Two's complement binary for -6148914691236517205 is 1010101010101010101010101010101010101010101010101010101010101011
        let result: i128 = a.into();
        assert_eq!(result, -6148914691236517205);
    }

    #[test]
    fn test_from_positive_i128() {
        let a: GarbledInt128 = 6148914691236517205_i128.into(); // Binary for 6148914691236517205 is 0101010101010101010101010101010101010101010101010101010101010101
        let result: i128 = a.into();
        assert_eq!(result, 6148914691236517205);
    }

    #[test]
    fn test_from_uint_to_int_i8() {
        let uint: GarbledUint8 = 170_u8.into(); // 10101010 (unsigned)
        let int: GarbledInt8 = uint.into(); // Interpreted as -86 (two's complement signed)
        let result: i8 = int.into();
        assert_eq!(result, 170_u8 as i8);
    }

    #[test]
    fn test_from_uint_to_int_i16() {
        let uint: GarbledUint16 = 43707_u16.into(); // 1010101010101011 (unsigned)
        let int: GarbledInt16 = uint.into(); // Interpreted as -21845 (two's complement signed)
        let result: i16 = int.into();
        assert_eq!(result, 43707_u16 as i16);
    }

    #[test]
    fn test_from_uint_to_int_i32() {
        let uint: GarbledUint32 = 2863311530_u32.into(); // 10101010101010101010101010101010 (unsigned)
        let int: GarbledInt32 = uint.into(); // Interpreted as -1431655766 (two's complement signed)
        let result: i32 = int.into();
        assert_eq!(result, 2863311530_u32 as i32);
    }

    #[test]
    fn test_from_uint_to_int_i64() {
        let uint: GarbledUint64 = 12297829382473034410_u64.into(); // 1010101010101010101010101010101010101010101010101010101010101010 (unsigned)
        let int: GarbledInt64 = uint.into(); // Interpreted as -6148914691236517206 (two's complement signed)
        let result: i64 = int.into();
        assert_eq!(result, 12297829382473034410_u64 as i64);
    }

    #[test]
    fn test_from_uint_to_int_i128() {
        let uint: GarbledUint128 = 12297829382473034410_u128.into(); // 1010101010101010101010101010101010101010101010101010101010101010 (unsigned)
        let int: GarbledInt128 = uint.into(); // Interpreted as -6148914691236517206 (two's complement signed)
        let result: i128 = int.into();
        assert_eq!(result, 12297829382473034410_u128 as i128);
    }
}
