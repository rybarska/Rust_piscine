#![forbid(unsafe_op_in_unsafe_fn)]
use std::slice;
use std::mem;

pub type GoldNugget = u16;
pub type Gold = [GoldNugget];

pub type Iron = u32;
pub type Mercure = u64;

pub struct PhilosopherStone;

/// Types that can be safely transmuted into GoldNugget arrays.
///
/// # Safety
///
/// Implementors of this trait must ensure that the memory layout
/// of their types conforms to the expected layout for `GoldNugget`,
/// allowing for safe transmutation to an array of GoldNuggets.
pub unsafe trait Metal {}

unsafe impl Metal for Iron{}

unsafe impl Metal for u16{}

unsafe impl Metal for Mercure{}

impl PhilosopherStone {
    /// Transmute an Iron type into an array of GoldNuggets.
    ///
    /// # Safety
    ///
    /// The size of `iron` must be 4 bytes (the size of `Iron`), 
    /// and the first 2 bytes of `iron` must be a valid `GoldNugget`.
    pub fn transmute_iron(self, iron: Iron) -> [GoldNugget; 2]
    {
        // Ensure the size of Iron is as expected (4 bytes).
        assert_eq!(4, mem::size_of_val(&iron));
        // SAFETY:
        // The size of `Iron` is guaranteed to be 4 bytes, which can be safely transmuted to two GoldNuggets.
        // The transmutation assumes that the first 2 bytes of `Iron` are a valid representation of a GoldNugget.
        let gold_nuggets: [GoldNugget; 2] = unsafe { std::mem::transmute(iron) };
        
        assert_eq!(gold_nuggets[0], iron as u16);
        gold_nuggets
    }
    /// Transmute a Mercure type into an array of GoldNuggets.
    ///
    /// # Safety
    ///
    /// The size of `mercure` must be 8 bytes (the size of `Mercure`), 
    /// and the first 2 bytes of `mercure` must be a valid `GoldNugget`.
    pub fn transmute_mercure(self, mercure: Mercure) -> [GoldNugget; 4]
    {
        // Ensure the size of Mercure is as expected (8 bytes).
        assert_eq!(8, mem::size_of_val(&mercure));
        // SAFETY:
        // The size of `Mercure` is guaranteed to be 8 bytes, which can be safely transmuted to four GoldNuggets.
        // The transmutation assumes that the first 2 bytes of `Mercure` are a valid representation of a GoldNugget.
        let gold_nuggets: [GoldNugget; 4] = unsafe { std::mem::transmute(mercure) };
        // Check that the first GoldNugget is equal to the lower 16 bits of Mercure.
        assert_eq!(gold_nuggets[0], mercure as u16);
        gold_nuggets
    }
    /// Transmute a generic type M (that implements Metal) into GoldNuggets.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the memory layout of `M` is compatible with `GoldNugget`.
    pub fn transmute_metal<M: Metal>(self, metal: &M) -> &Gold
    {
        // Determine the size of the Metal type.
        let metal_size = mem::size_of_val(metal);
        // Calculate the number of GoldNuggets that this metal can be transmuted into.
        let num_of_nuggets = metal_size / mem::size_of::<GoldNugget>();
        // SAFETY:
        // The pointer `ptr` is derived from a reference to `metal`, which is assumed to be valid.
        // We also assume that `metal` has been correctly aligned for a `GoldNugget` array.
        let gold_nuggets: &[GoldNugget] = {
            let ptr = metal as *const M as *const GoldNugget;
            let slice = unsafe { slice::from_raw_parts(ptr, num_of_nuggets) };
            slice
        };
        gold_nuggets
    }
}