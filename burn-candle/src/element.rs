use std::borrow::Borrow;

use burn_tensor::Element;
use candle_core::{Tensor, WithDType};
use half::{bf16, f16};

pub trait CandleElement: Element + WithDType {}
pub trait FloatCandleElement: CandleElement {}
pub trait IntCandleElement: CandleElement {}

impl CandleElement for f64 {}
impl FloatCandleElement for f64 {}

impl CandleElement for f32 {}
impl FloatCandleElement for f32 {}

impl CandleElement for f16 {}
impl FloatCandleElement for f16 {}

impl CandleElement for bf16 {}
impl FloatCandleElement for bf16 {}

impl CandleElement for u8 {}
impl IntCandleElement for u8 {}
//maybe also a quantization type?

impl CandleElement for u32 {}
impl IntCandleElement for u32 {}

// Support for i32 may come soon (https://github.com/huggingface/candle/issues/514)
