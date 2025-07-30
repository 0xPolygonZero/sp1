use sp1_derive::AlignedBorrow;
use sp1_stark::Word;

/// Memory read access.
#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryReadCols<T> {
    pub access: MemoryAccessCols<T>,
}

/// Memory write access.
#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryWriteCols<T> {
    pub prev_value: Word<T>,
    pub access: MemoryAccessCols<T>,
}

/// Memory write access, without access value.
#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryWriteColsNoVal<T> {
    pub prev_value: Word<T>,
    pub access: MemoryAccessColsNoVal<T>,
}

/// Memory read-write access.
#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryReadWriteCols<T> {
    pub prev_value: Word<T>,
    pub access: MemoryAccessCols<T>,
}

#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryAccessCols<T> {
    /// The value of the memory access.
    pub value: Word<T>,

    /// The previous shard and timestamp that this memory access is being read from.
    pub prev_shard: T,
    pub prev_clk: T,

    /// This will be true if the current shard == prev_access's shard, else false.
    pub compare_clk: T,

    /// The following columns are decomposed limbs for the difference between the current access's
    /// timestamp and the previous access's timestamp.  Note the actual value of the timestamp
    /// is either the accesses' shard or clk depending on the value of compare_clk.
    ///
    /// This column is the least significant 16 bit limb of current access timestamp - prev access
    /// timestamp.
    pub diff_16bit_limb: T,

    /// This column is the most significant 8 bit limb of current access timestamp - prev access
    /// timestamp.
    pub diff_8bit_limb: T,
}

#[derive(AlignedBorrow, Default, Debug, Clone, Copy)]
#[repr(C)]
pub struct MemoryAccessColsNoVal<T> {
    /// The previous shard and timestamp that this memory access is being read from.
    pub prev_shard: T,
    pub prev_clk: T,

    /// This will be true if the current shard == prev_access's shard, else false.
    pub compare_clk: T,

    /// The following columns are decomposed limbs for the difference between the current access's
    /// timestamp and the previous access's timestamp.  Note the actual value of the timestamp
    /// is either the accesses' shard or clk depending on the value of compare_clk.
    ///
    /// This column is the least significant 16 bit limb of current access timestamp - prev access
    /// timestamp.
    pub diff_16bit_limb: T,

    /// This column is the most significant 8 bit limb of current access timestamp - prev access
    /// timestamp.
    pub diff_8bit_limb: T,
}

impl<T> MemoryAccessCols<T> {
    /// Sets `MemoryAccessCols` from a value and a `MemoryAccessColsNoVal`.
    pub fn new_from_val_and_no_val(value: Word<T>, mem_no_val: MemoryAccessColsNoVal<T>) -> Self {
        Self {
            value,
            prev_shard: mem_no_val.prev_shard,
            prev_clk: mem_no_val.prev_clk,
            compare_clk: mem_no_val.compare_clk,
            diff_16bit_limb: mem_no_val.diff_16bit_limb,
            diff_8bit_limb: mem_no_val.diff_8bit_limb,
        }
    }
}

/// The common columns for all memory access types.
pub trait MemoryCols<T> {
    fn access(&self) -> &MemoryAccessCols<T>;

    fn access_mut(&mut self) -> &mut MemoryAccessCols<T>;

    fn prev_value(&self) -> &Word<T>;

    fn prev_value_mut(&mut self) -> &mut Word<T>;

    fn value(&self) -> &Word<T>;

    fn value_mut(&mut self) -> &mut Word<T>;
}

/// The common columns for all memory access types which do not include the value.
pub trait MemoryColsNoVal<T> {
    fn access(&self) -> &MemoryAccessColsNoVal<T>;

    fn access_mut(&mut self) -> &mut MemoryAccessColsNoVal<T>;

    fn prev_value(&self) -> &Word<T>;

    fn prev_value_mut(&mut self) -> &mut Word<T>;
}

impl<T> MemoryCols<T> for MemoryReadCols<T> {
    fn access(&self) -> &MemoryAccessCols<T> {
        &self.access
    }

    fn access_mut(&mut self) -> &mut MemoryAccessCols<T> {
        &mut self.access
    }

    fn prev_value(&self) -> &Word<T> {
        &self.access.value
    }

    fn prev_value_mut(&mut self) -> &mut Word<T> {
        &mut self.access.value
    }

    fn value(&self) -> &Word<T> {
        &self.access.value
    }

    fn value_mut(&mut self) -> &mut Word<T> {
        &mut self.access.value
    }
}

impl<T> MemoryCols<T> for MemoryWriteCols<T> {
    fn access(&self) -> &MemoryAccessCols<T> {
        &self.access
    }

    fn access_mut(&mut self) -> &mut MemoryAccessCols<T> {
        &mut self.access
    }

    fn prev_value(&self) -> &Word<T> {
        &self.prev_value
    }

    fn prev_value_mut(&mut self) -> &mut Word<T> {
        &mut self.prev_value
    }

    fn value(&self) -> &Word<T> {
        &self.access.value
    }

    fn value_mut(&mut self) -> &mut Word<T> {
        &mut self.access.value
    }
}

impl<T> MemoryColsNoVal<T> for MemoryWriteColsNoVal<T> {
    fn access(&self) -> &MemoryAccessColsNoVal<T> {
        &self.access
    }

    fn access_mut(&mut self) -> &mut MemoryAccessColsNoVal<T> {
        &mut self.access
    }

    fn prev_value(&self) -> &Word<T> {
        &self.prev_value
    }

    fn prev_value_mut(&mut self) -> &mut Word<T> {
        &mut self.prev_value
    }
}

impl<T> MemoryCols<T> for MemoryReadWriteCols<T> {
    fn access(&self) -> &MemoryAccessCols<T> {
        &self.access
    }

    fn access_mut(&mut self) -> &mut MemoryAccessCols<T> {
        &mut self.access
    }

    fn prev_value(&self) -> &Word<T> {
        &self.prev_value
    }

    fn prev_value_mut(&mut self) -> &mut Word<T> {
        &mut self.prev_value
    }

    fn value(&self) -> &Word<T> {
        &self.access.value
    }

    fn value_mut(&mut self) -> &mut Word<T> {
        &mut self.access.value
    }
}

/// A utility method to convert a slice of memory access columns into a vector of values.
/// This is useful for comparing the values of a memory access to limbs.
pub fn value_as_limbs<T: Clone, M: MemoryCols<T>>(memory: &[M]) -> Vec<T> {
    memory.iter().flat_map(|m| m.value().clone().into_iter()).collect()
}

pub fn memory_cols_vec_from_no_vals<T: Clone>(
    all_values: &[Word<T>],
    mem_no_val: &[MemoryWriteColsNoVal<T>],
) -> Vec<MemoryWriteCols<T>> {
    (0..mem_no_val.len())
        .map(|i| MemoryWriteCols {
            prev_value: mem_no_val[i].prev_value.clone(),
            access: MemoryAccessCols::new_from_val_and_no_val(
                all_values[i].clone(),
                mem_no_val[i].access.clone(),
            ),
        })
        .collect()
}

pub fn slice_to_words<T: Copy>(slice: &[T]) -> Vec<Word<T>> {
    slice
        .chunks_exact(sp1_primitives::consts::WORD_SIZE)
        .map(|chunk| Word(chunk.try_into().unwrap()))
        .collect()
}
