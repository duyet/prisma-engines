//! Note: Only number operations are implemented at the moment.
use super::*;
use std::ops::*;

/// Used right now to reduce code duplication, probably needs to be scrapped once we need anything beyond that.
macro_rules! number_operation {
  ($trait_:ident, $fname:ident, $op:tt) => {
    impl $trait_ for PrismaValue {
      type Output = PrismaValue;

      fn $fname(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
          (PrismaValue::Null, _) | (_, PrismaValue::Null) => PrismaValue::Null,
          (PrismaValue::Int(l), PrismaValue::Int(r)) => PrismaValue::Int(l $op r),
          (PrismaValue::BigInt(l), PrismaValue::BigInt(r)) => PrismaValue::BigInt(l $op r),

          (PrismaValue::Int(l), PrismaValue::BigInt(r)) => PrismaValue::Int(l $op r),
          (PrismaValue::BigInt(l), PrismaValue::Int(r)) => PrismaValue::BigInt(l $op r),

          (PrismaValue::Float(l), PrismaValue::Float(r)) => PrismaValue::Float(FloatValue(*l $op *r)),
          (PrismaValue::Decimal(l), PrismaValue::Decimal(r)) => PrismaValue::Decimal(l $op r),

          // (PrismaValue::Float(l), PrismaValue::Int(r)) => PrismaValue::Float(FloatValue(l.0 $op r)),

          _ => unimplemented!(),
        }
      }
    }
  }
}

number_operation!(Add, add, +);
number_operation!(Sub, sub, -);
number_operation!(Div, div, /);
number_operation!(Mul, mul, *);
