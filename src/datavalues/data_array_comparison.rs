// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use std::sync::Arc;

use crate::datavalues::{DataArrayRef, DataColumnarValue, DataType, DataValueComparisonOperator};
use crate::datavalues::{
    Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, StringArray,
    UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};
use crate::error::{FuseQueryError, FuseQueryResult};

pub fn data_array_comparison_op(
    op: DataValueComparisonOperator,
    left: &DataColumnarValue,
    right: &DataColumnarValue,
) -> FuseQueryResult<DataArrayRef> {
    match (left, right) {
        (DataColumnarValue::Array(left_array), DataColumnarValue::Array(right_array)) => match op {
            DataValueComparisonOperator::Eq => arrow_array_op!(&left_array, &right_array, eq),
            DataValueComparisonOperator::Lt => arrow_array_op!(&left_array, &right_array, lt),
            DataValueComparisonOperator::LtEq => arrow_array_op!(&left_array, &right_array, lt_eq),
            DataValueComparisonOperator::Gt => arrow_array_op!(&left_array, &right_array, gt),
            DataValueComparisonOperator::GtEq => arrow_array_op!(&left_array, &right_array, gt_eq),
        },

        (DataColumnarValue::Array(array), DataColumnarValue::Scalar(scalar)) => match op {
            DataValueComparisonOperator::Eq => arrow_array_op_scalar!(array, scalar.clone(), eq),
            DataValueComparisonOperator::Lt => arrow_array_op_scalar!(array, scalar.clone(), lt),
            DataValueComparisonOperator::LtEq => {
                arrow_array_op_scalar!(array, scalar.clone(), lt_eq)
            }
            DataValueComparisonOperator::Gt => arrow_array_op_scalar!(array, scalar.clone(), gt),
            DataValueComparisonOperator::GtEq => {
                arrow_array_op_scalar!(array, scalar.clone(), gt_eq)
            }
        },

        (DataColumnarValue::Scalar(scalar), DataColumnarValue::Array(array)) => match op {
            DataValueComparisonOperator::Eq => arrow_array_op_scalar!(array, scalar.clone(), eq),
            DataValueComparisonOperator::Lt => arrow_array_op_scalar!(array, scalar.clone(), gt),
            DataValueComparisonOperator::LtEq => {
                arrow_array_op_scalar!(array, scalar.clone(), gt_eq)
            }
            DataValueComparisonOperator::Gt => arrow_array_op_scalar!(array, scalar.clone(), lt),
            DataValueComparisonOperator::GtEq => {
                arrow_array_op_scalar!(array, scalar.clone(), lt_eq)
            }
        },

        _ => Err(FuseQueryError::Internal(format!(
            "Cannot do data_array {}, left:{:?}, right:{:?}",
            op,
            left.data_type(),
            right.data_type()
        ))),
    }
}