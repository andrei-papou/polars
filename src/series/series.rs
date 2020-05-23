use super::chunked_array::ChunkedArray;
use crate::series::chunked_array::SeriesOps;
use crate::{datatypes, error::Result};
use arrow::array::ArrayRef;
use arrow::compute::TakeOptions;
use std::ops::Deref;

pub enum Series {
    Int32(ChunkedArray<datatypes::Int32Type>),
    Int64(ChunkedArray<datatypes::Int64Type>),
    Float32(ChunkedArray<datatypes::Float32Type>),
    Float64(ChunkedArray<datatypes::Float64Type>),
    Utf8(ChunkedArray<datatypes::Utf8Type>),
}

macro_rules! apply_method {
    ($self:ident, $method:ident, $($args:ident),*) => {
        match $self {
            Series::Int32(a) => a.$method($($args),*),
            Series::Int64(a) => a.$method($($args),*),
            Series::Float32(a) => a.$method($($args),*),
            Series::Float64(a) => a.$method($($args),*),
            Series::Utf8(a) => a.$method($($args),*),
            _ => unimplemented!(),
        }
    }
}

macro_rules! apply_method_and_return {
    ($self:ident, $method:ident, [$($args:expr),+], $($opt_question_mark:tt)*) => {
        match $self {
            Series::Int32(a) => Series::Int32(a.$method($($args),+)$($opt_question_mark)*),
            Series::Int64(a) => Series::Int64(a.$method($($args),+)$($opt_question_mark)*),
            Series::Float32(a) => Series::Float32(a.$method($($args),+)$($opt_question_mark)*),
            Series::Float64(a) => Series::Float64(a.$method($($args),+)$($opt_question_mark)*),
            Series::Utf8(a) => Series::Utf8(a.$method($($args),+)$($opt_question_mark)*),
            _ => unimplemented!(),
        }
    }
}

impl Series {
    pub fn append_array(&mut self, other: ArrayRef) -> Result<()> {
        apply_method!(self, append_array, other)
    }

    pub fn as_series_ops(&self) -> &dyn SeriesOps {
        match self {
            Series::Int32(arr) => arr,
            Series::Int64(arr) => arr,
            Series::Float32(arr) => arr,
            Series::Float64(arr) => arr,
            Series::Utf8(arr) => arr,
        }
    }

    fn limit(&self, num_elements: usize) -> Result<Self> {
        Ok(apply_method_and_return!(self, limit, [num_elements], ?))
    }
    fn filter(&self, filter: &ChunkedArray<datatypes::BooleanType>) -> Result<Self> {
        Ok(apply_method_and_return!(self, filter, [filter], ?))
    }
    fn take(
        &self,
        indices: &ChunkedArray<datatypes::UInt32Type>,
        options: Option<TakeOptions>,
    ) -> Result<Self> {
        Ok(apply_method_and_return!(self, take, [indices, options], ?))
    }

    fn len(&self) -> usize {
        apply_method!(self, len,)
    }
}