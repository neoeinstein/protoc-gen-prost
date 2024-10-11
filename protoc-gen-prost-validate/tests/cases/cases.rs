use super::gen::tests::harness::cases;
use super::gen::tests::harness::cases::message_required_oneof::One;
use super::gen::tests::harness::cases::*;
use super::{now, Factory};
use once_cell::sync::Lazy;
use prost_types::{Any, Duration, Timestamp};
use prost_validate::Validator;
use std::collections::HashMap;

pub static CASES: Lazy<HashMap<&'static str, Factory>> = Lazy::new(|| {
    HashMap::from([
        (
            "float_none_valid",
            Box::new(|| {
                (
                    Box::new(FloatNone { val: -1.23456 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "float_const_valid",
            Box::new(|| (Box::new(FloatConst { val: 1.23 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_const_invalid",
            Box::new(|| (Box::new(FloatConst { val: 4.56 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_in_valid",
            Box::new(|| (Box::new(FloatIn { val: 7.89 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_in_invalid",
            Box::new(|| (Box::new(FloatIn { val: 10.11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_not_in_valid",
            Box::new(|| (Box::new(FloatNotIn { val: 1. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_not_in_invalid",
            Box::new(|| (Box::new(FloatNotIn { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_lt_valid",
            Box::new(|| (Box::new(FloatLt { val: -1. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_lt_invalid_equal",
            Box::new(|| (Box::new(FloatLt { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_lt_invalid",
            Box::new(|| (Box::new(FloatLt { val: 1. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_lte_valid",
            Box::new(|| (Box::new(FloatLte { val: 63. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_lte_valid_equal",
            Box::new(|| (Box::new(FloatLte { val: 64. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_lte_invalid",
            Box::new(|| (Box::new(FloatLte { val: 65. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gt_valid",
            Box::new(|| (Box::new(FloatGt { val: 17. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gt_invalid_equal",
            Box::new(|| (Box::new(FloatGt { val: 16. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gt_invalid",
            Box::new(|| (Box::new(FloatGt { val: 15. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gte_valid",
            Box::new(|| (Box::new(FloatGte { val: 9. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gte_valid_equal",
            Box::new(|| (Box::new(FloatGte { val: 8. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gte_invalid",
            Box::new(|| (Box::new(FloatGte { val: 7. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gt_lt_valid",
            Box::new(|| (Box::new(FloatGtlt { val: 5. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gt_lt_invalid_above",
            Box::new(|| (Box::new(FloatGtlt { val: 11. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gt_lt_invalid_below",
            Box::new(|| (Box::new(FloatGtlt { val: -1. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gt_lt_invalid_max",
            Box::new(|| (Box::new(FloatGtlt { val: 10. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gt_lt_invalid_min",
            Box::new(|| (Box::new(FloatGtlt { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(FloatExLtgt { val: 11. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(FloatExLtgt { val: -1. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(FloatExLtgt { val: 5. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(FloatExLtgt { val: 10. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(FloatExLtgt { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gte_lte_valid",
            Box::new(|| (Box::new(FloatGtelte { val: 200. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gte_lte_valid_max",
            Box::new(|| (Box::new(FloatGtelte { val: 256. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gte_lte_valid_min",
            Box::new(|| (Box::new(FloatGtelte { val: 128. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "float_gte_lte_invalid_above",
            Box::new(|| (Box::new(FloatGtelte { val: 300. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_gte_lte_invalid_below",
            Box::new(|| (Box::new(FloatGtelte { val: 100. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "float_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(FloatExGtelte { val: 300. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "float_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(FloatExGtelte { val: 100. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "float_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(FloatExGtelte { val: 256. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "float_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(FloatExGtelte { val: 128. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "float_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(FloatExGtelte { val: 200. }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "float_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(FloatIgnore { val: 0. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_none_valid",
            Box::new(|| {
                (
                    Box::new(DoubleNone { val: -1.23456 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_const_valid",
            Box::new(|| (Box::new(DoubleConst { val: 1.23 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_const_invalid",
            Box::new(|| (Box::new(DoubleConst { val: 4.56 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_in_valid",
            Box::new(|| (Box::new(DoubleIn { val: 7.89 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_in_invalid",
            Box::new(|| (Box::new(DoubleIn { val: 10.11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_not_in_valid",
            Box::new(|| (Box::new(DoubleNotIn { val: 1. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_not_in_invalid",
            Box::new(|| (Box::new(DoubleNotIn { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_lt_valid",
            Box::new(|| (Box::new(DoubleLt { val: -1. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_lt_invalid_equal",
            Box::new(|| (Box::new(DoubleLt { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_lt_invalid",
            Box::new(|| (Box::new(DoubleLt { val: 1. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_lte_valid",
            Box::new(|| (Box::new(DoubleLte { val: 63. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_lte_valid_equal",
            Box::new(|| (Box::new(DoubleLte { val: 64. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_lte_invalid",
            Box::new(|| (Box::new(DoubleLte { val: 65. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gt_valid",
            Box::new(|| (Box::new(DoubleGt { val: 17. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_gt_invalid_equal",
            Box::new(|| (Box::new(DoubleGt { val: 16. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gt_invalid",
            Box::new(|| (Box::new(DoubleGt { val: 15. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gte_valid",
            Box::new(|| (Box::new(DoubleGte { val: 9. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_gte_valid_equal",
            Box::new(|| (Box::new(DoubleGte { val: 8. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_gte_invalid",
            Box::new(|| (Box::new(DoubleGte { val: 7. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gt_lt_valid",
            Box::new(|| (Box::new(DoubleGtlt { val: 5. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_gt_lt_invalid_above",
            Box::new(|| (Box::new(DoubleGtlt { val: 11. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gt_lt_invalid_below",
            Box::new(|| (Box::new(DoubleGtlt { val: -1. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gt_lt_invalid_max",
            Box::new(|| (Box::new(DoubleGtlt { val: 10. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gt_lt_invalid_min",
            Box::new(|| (Box::new(DoubleGtlt { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(DoubleExLtgt { val: 11. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(DoubleExLtgt { val: -1. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "double_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(DoubleExLtgt { val: 5. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(DoubleExLtgt { val: 10. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(DoubleExLtgt { val: 0. }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "double_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(DoubleGtelte { val: 200. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(DoubleGtelte { val: 256. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(DoubleGtelte { val: 128. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(DoubleGtelte { val: 300. }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "double_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(DoubleGtelte { val: 100. }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "double_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(DoubleExGtelte { val: 300. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(DoubleExGtelte { val: 100. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(DoubleExGtelte { val: 256. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(DoubleExGtelte { val: 128. }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "double_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(DoubleExGtelte { val: 200. }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "double_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(DoubleIgnore { val: 0. }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_none_valid",
            Box::new(|| (Box::new(Int32None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_const_valid",
            Box::new(|| (Box::new(Int32Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_const_invalid",
            Box::new(|| (Box::new(Int32Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_in_valid",
            Box::new(|| (Box::new(Int32In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_in_invalid",
            Box::new(|| (Box::new(Int32In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_not_in_valid",
            Box::new(|| (Box::new(Int32NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_not_in_invalid",
            Box::new(|| (Box::new(Int32NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_lt_valid",
            Box::new(|| (Box::new(Int32Lt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_lt_invalid_equal",
            Box::new(|| (Box::new(Int32Lt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_lt_invalid",
            Box::new(|| (Box::new(Int32Lt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_lte_valid",
            Box::new(|| (Box::new(Int32Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_lte_valid_equal",
            Box::new(|| (Box::new(Int32Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_lte_invalid",
            Box::new(|| (Box::new(Int32Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gt_valid",
            Box::new(|| (Box::new(Int32Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gt_invalid_equal",
            Box::new(|| (Box::new(Int32Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gt_invalid",
            Box::new(|| (Box::new(Int32Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gte_valid",
            Box::new(|| (Box::new(Int32Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gte_valid_equal",
            Box::new(|| (Box::new(Int32Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gte_invalid",
            Box::new(|| (Box::new(Int32Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gt_lt_valid",
            Box::new(|| (Box::new(Int32Gtlt { val: 5 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gt_lt_invalid_above",
            Box::new(|| (Box::new(Int32Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gt_lt_invalid_below",
            Box::new(|| (Box::new(Int32Gtlt { val: -1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gt_lt_invalid_max",
            Box::new(|| (Box::new(Int32Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gt_lt_invalid_min",
            Box::new(|| (Box::new(Int32Gtlt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(Int32ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(Int32ExLtgt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(Int32ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(Int32ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(Int32ExLtgt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gte_lte_valid",
            Box::new(|| (Box::new(Int32Gtelte { val: 200 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gte_lte_valid_max",
            Box::new(|| (Box::new(Int32Gtelte { val: 256 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gte_lte_valid_min",
            Box::new(|| (Box::new(Int32Gtelte { val: 128 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int32_gte_lte_invalid_above",
            Box::new(|| (Box::new(Int32Gtelte { val: 300 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_gte_lte_invalid_below",
            Box::new(|| (Box::new(Int32Gtelte { val: 100 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int32_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(Int32ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int32_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(Int32ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int32_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(Int32ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int32_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(Int32ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int32_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(Int32ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "int32_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(Int32Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_none_valid",
            Box::new(|| (Box::new(Int64None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_const_valid",
            Box::new(|| (Box::new(Int64Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_const_invalid",
            Box::new(|| (Box::new(Int64Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_in_valid",
            Box::new(|| (Box::new(Int64In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_in_invalid",
            Box::new(|| (Box::new(Int64In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_not_in_valid",
            Box::new(|| (Box::new(Int64NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_not_in_invalid",
            Box::new(|| (Box::new(Int64NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_lt_valid",
            Box::new(|| (Box::new(Int64Lt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_lt_invalid_equal",
            Box::new(|| (Box::new(Int64Lt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_lt_invalid",
            Box::new(|| (Box::new(Int64Lt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_lte_valid",
            Box::new(|| (Box::new(Int64Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_lte_valid_equal",
            Box::new(|| (Box::new(Int64Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_lte_invalid",
            Box::new(|| (Box::new(Int64Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gt_valid",
            Box::new(|| (Box::new(Int64Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gt_invalid_equal",
            Box::new(|| (Box::new(Int64Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gt_invalid",
            Box::new(|| (Box::new(Int64Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gte_valid",
            Box::new(|| (Box::new(Int64Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gte_valid_equal",
            Box::new(|| (Box::new(Int64Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gte_invalid",
            Box::new(|| (Box::new(Int64Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gt_lt_valid",
            Box::new(|| (Box::new(Int64Gtlt { val: 5 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gt_lt_invalid_above",
            Box::new(|| (Box::new(Int64Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gt_lt_invalid_below",
            Box::new(|| (Box::new(Int64Gtlt { val: -1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gt_lt_invalid_max",
            Box::new(|| (Box::new(Int64Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gt_lt_invalid_min",
            Box::new(|| (Box::new(Int64Gtlt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(Int64ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(Int64ExLtgt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(Int64ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(Int64ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(Int64ExLtgt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gte_lte_valid",
            Box::new(|| (Box::new(Int64Gtelte { val: 200 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gte_lte_valid_max",
            Box::new(|| (Box::new(Int64Gtelte { val: 256 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gte_lte_valid_min",
            Box::new(|| (Box::new(Int64Gtelte { val: 128 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_gte_lte_invalid_above",
            Box::new(|| (Box::new(Int64Gtelte { val: 300 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_gte_lte_invalid_below",
            Box::new(|| (Box::new(Int64Gtelte { val: 100 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "int64_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(Int64ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int64_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(Int64ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int64_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(Int64ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int64_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(Int64ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int64_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(Int64ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "int64_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(Int64Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "int64_optional_lte_valid",
            Box::new(|| {
                (
                    Box::new(Int64LteOptional { val: Some(63) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int64_optional_lte_valid_equal",
            Box::new(|| {
                (
                    Box::new(Int64LteOptional { val: Some(64) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "int64_optional_lte_valid_unset",
            Box::new(|| {
                (
                    Box::new(Int64LteOptional { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint32_none_valid",
            Box::new(|| (Box::new(UInt32None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_const_valid",
            Box::new(|| (Box::new(UInt32Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_const_invalid",
            Box::new(|| (Box::new(UInt32Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_in_valid",
            Box::new(|| (Box::new(UInt32In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_in_invalid",
            Box::new(|| (Box::new(UInt32In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_not_in_valid",
            Box::new(|| (Box::new(UInt32NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_not_in_invalid",
            Box::new(|| (Box::new(UInt32NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_lt_valid",
            Box::new(|| (Box::new(UInt32Lt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_lt_invalid_equal",
            Box::new(|| (Box::new(UInt32Lt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_lt_invalid",
            Box::new(|| (Box::new(UInt32Lt { val: 6 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_lte_valid",
            Box::new(|| (Box::new(UInt32Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_lte_valid_equal",
            Box::new(|| (Box::new(UInt32Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_lte_invalid",
            Box::new(|| (Box::new(UInt32Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gt_valid",
            Box::new(|| (Box::new(UInt32Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gt_invalid_equal",
            Box::new(|| (Box::new(UInt32Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gt_invalid",
            Box::new(|| (Box::new(UInt32Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gte_valid",
            Box::new(|| (Box::new(UInt32Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gte_valid_equal",
            Box::new(|| (Box::new(UInt32Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gte_invalid",
            Box::new(|| (Box::new(UInt32Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gt_lt_valid",
            Box::new(|| (Box::new(UInt32Gtlt { val: 7 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gt_lt_invalid_above",
            Box::new(|| (Box::new(UInt32Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gt_lt_invalid_below",
            Box::new(|| (Box::new(UInt32Gtlt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gt_lt_invalid_max",
            Box::new(|| (Box::new(UInt32Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gt_lt_invalid_min",
            Box::new(|| (Box::new(UInt32Gtlt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(UInt32ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(UInt32ExLtgt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(UInt32ExLtgt { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(UInt32ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(UInt32ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gte_lte_valid",
            Box::new(|| (Box::new(UInt32Gtelte { val: 200 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gte_lte_valid_max",
            Box::new(|| (Box::new(UInt32Gtelte { val: 256 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gte_lte_valid_min",
            Box::new(|| (Box::new(UInt32Gtelte { val: 128 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint32_gte_lte_invalid_above",
            Box::new(|| (Box::new(UInt32Gtelte { val: 300 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_gte_lte_invalid_below",
            Box::new(|| (Box::new(UInt32Gtelte { val: 100 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint32_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(UInt32ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint32_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(UInt32ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint32_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(UInt32ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint32_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(UInt32ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint32_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(UInt32ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "uint32_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(UInt32Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_none_valid",
            Box::new(|| (Box::new(UInt64None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_const_valid",
            Box::new(|| (Box::new(UInt64Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_const_invalid",
            Box::new(|| (Box::new(UInt64Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_in_valid",
            Box::new(|| (Box::new(UInt64In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_in_invalid",
            Box::new(|| (Box::new(UInt64In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_not_in_valid",
            Box::new(|| (Box::new(UInt64NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_not_in_invalid",
            Box::new(|| (Box::new(UInt64NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_lt_valid",
            Box::new(|| (Box::new(UInt64Lt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_lt_invalid_equal",
            Box::new(|| (Box::new(UInt64Lt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_lt_invalid",
            Box::new(|| (Box::new(UInt64Lt { val: 6 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_lte_valid",
            Box::new(|| (Box::new(UInt64Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_lte_valid_equal",
            Box::new(|| (Box::new(UInt64Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_lte_invalid",
            Box::new(|| (Box::new(UInt64Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gt_valid",
            Box::new(|| (Box::new(UInt64Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gt_invalid_equal",
            Box::new(|| (Box::new(UInt64Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gt_invalid",
            Box::new(|| (Box::new(UInt64Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gte_valid",
            Box::new(|| (Box::new(UInt64Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gte_valid_equal",
            Box::new(|| (Box::new(UInt64Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gte_invalid",
            Box::new(|| (Box::new(UInt64Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gt_lt_valid",
            Box::new(|| (Box::new(UInt64Gtlt { val: 7 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gt_lt_invalid_above",
            Box::new(|| (Box::new(UInt64Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gt_lt_invalid_below",
            Box::new(|| (Box::new(UInt64Gtlt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gt_lt_invalid_max",
            Box::new(|| (Box::new(UInt64Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gt_lt_invalid_min",
            Box::new(|| (Box::new(UInt64Gtlt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(UInt64ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(UInt64ExLtgt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(UInt64ExLtgt { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(UInt64ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(UInt64ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gte_lte_valid",
            Box::new(|| (Box::new(UInt64Gtelte { val: 200 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gte_lte_valid_max",
            Box::new(|| (Box::new(UInt64Gtelte { val: 256 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gte_lte_valid_min",
            Box::new(|| (Box::new(UInt64Gtelte { val: 128 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "uint64_gte_lte_invalid_above",
            Box::new(|| (Box::new(UInt64Gtelte { val: 300 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_gte_lte_invalid_below",
            Box::new(|| (Box::new(UInt64Gtelte { val: 100 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "uint64_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(UInt64ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint64_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(UInt64ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint64_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(UInt64ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint64_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(UInt64ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "uint64_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(UInt64ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "uint64_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(UInt64Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_none_valid",
            Box::new(|| (Box::new(SInt32None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_const_valid",
            Box::new(|| (Box::new(SInt32Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_const_invalid",
            Box::new(|| (Box::new(SInt32Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_in_valid",
            Box::new(|| (Box::new(SInt32In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_in_invalid",
            Box::new(|| (Box::new(SInt32In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_not_in_valid",
            Box::new(|| (Box::new(SInt32NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_not_in_invalid",
            Box::new(|| (Box::new(SInt32NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_lt_valid",
            Box::new(|| (Box::new(SInt32Lt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_lt_invalid_equal",
            Box::new(|| (Box::new(SInt32Lt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_lt_invalid",
            Box::new(|| (Box::new(SInt32Lt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_lte_valid",
            Box::new(|| (Box::new(SInt32Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_lte_valid_equal",
            Box::new(|| (Box::new(SInt32Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_lte_invalid",
            Box::new(|| (Box::new(SInt32Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gt_valid",
            Box::new(|| (Box::new(SInt32Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gt_invalid_equal",
            Box::new(|| (Box::new(SInt32Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gt_invalid",
            Box::new(|| (Box::new(SInt32Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gte_valid",
            Box::new(|| (Box::new(SInt32Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gte_valid_equal",
            Box::new(|| (Box::new(SInt32Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gte_invalid",
            Box::new(|| (Box::new(SInt32Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gt_lt_valid",
            Box::new(|| (Box::new(SInt32Gtlt { val: 5 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gt_lt_invalid_above",
            Box::new(|| (Box::new(SInt32Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gt_lt_invalid_below",
            Box::new(|| (Box::new(SInt32Gtlt { val: -1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gt_lt_invalid_max",
            Box::new(|| (Box::new(SInt32Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gt_lt_invalid_min",
            Box::new(|| (Box::new(SInt32Gtlt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(SInt32ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(SInt32ExLtgt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(SInt32ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(SInt32ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(SInt32ExLtgt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gte_lte_valid",
            Box::new(|| (Box::new(SInt32Gtelte { val: 200 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gte_lte_valid_max",
            Box::new(|| (Box::new(SInt32Gtelte { val: 256 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gte_lte_valid_min",
            Box::new(|| (Box::new(SInt32Gtelte { val: 128 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint32_gte_lte_invalid_above",
            Box::new(|| (Box::new(SInt32Gtelte { val: 300 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_gte_lte_invalid_below",
            Box::new(|| (Box::new(SInt32Gtelte { val: 100 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint32_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(SInt32ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint32_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(SInt32ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint32_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(SInt32ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint32_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(SInt32ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint32_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(SInt32ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sint32_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(SInt32Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_none_valid",
            Box::new(|| (Box::new(SInt64None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_const_valid",
            Box::new(|| (Box::new(SInt64Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_const_invalid",
            Box::new(|| (Box::new(SInt64Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_in_valid",
            Box::new(|| (Box::new(SInt64In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_in_invalid",
            Box::new(|| (Box::new(SInt64In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_not_in_valid",
            Box::new(|| (Box::new(SInt64NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_not_in_invalid",
            Box::new(|| (Box::new(SInt64NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_lt_valid",
            Box::new(|| (Box::new(SInt64Lt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_lt_invalid_equal",
            Box::new(|| (Box::new(SInt64Lt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_lt_invalid",
            Box::new(|| (Box::new(SInt64Lt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_lte_valid",
            Box::new(|| (Box::new(SInt64Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_lte_valid_equal",
            Box::new(|| (Box::new(SInt64Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_lte_invalid",
            Box::new(|| (Box::new(SInt64Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gt_valid",
            Box::new(|| (Box::new(SInt64Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gt_invalid_equal",
            Box::new(|| (Box::new(SInt64Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gt_invalid",
            Box::new(|| (Box::new(SInt64Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gte_valid",
            Box::new(|| (Box::new(SInt64Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gte_valid_equal",
            Box::new(|| (Box::new(SInt64Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gte_invalid",
            Box::new(|| (Box::new(SInt64Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gt_lt_valid",
            Box::new(|| (Box::new(SInt64Gtlt { val: 5 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gt_lt_invalid_above",
            Box::new(|| (Box::new(SInt64Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gt_lt_invalid_below",
            Box::new(|| (Box::new(SInt64Gtlt { val: -1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gt_lt_invalid_max",
            Box::new(|| (Box::new(SInt64Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gt_lt_invalid_min",
            Box::new(|| (Box::new(SInt64Gtlt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(SInt64ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(SInt64ExLtgt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(SInt64ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(SInt64ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(SInt64ExLtgt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gte_lte_valid",
            Box::new(|| (Box::new(SInt64Gtelte { val: 200 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gte_lte_valid_max",
            Box::new(|| (Box::new(SInt64Gtelte { val: 256 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gte_lte_valid_min",
            Box::new(|| (Box::new(SInt64Gtelte { val: 128 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sint64_gte_lte_invalid_above",
            Box::new(|| (Box::new(SInt64Gtelte { val: 300 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_gte_lte_invalid_below",
            Box::new(|| (Box::new(SInt64Gtelte { val: 100 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sint64_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(SInt64ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint64_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(SInt64ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint64_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(SInt64ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint64_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(SInt64ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sint64_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(SInt64ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sint64_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(SInt64Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_none_valid",
            Box::new(|| (Box::new(Fixed32None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_const_valid",
            Box::new(|| (Box::new(Fixed32Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_const_invalid",
            Box::new(|| (Box::new(Fixed32Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_in_valid",
            Box::new(|| (Box::new(Fixed32In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_in_invalid",
            Box::new(|| (Box::new(Fixed32In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_not_in_valid",
            Box::new(|| (Box::new(Fixed32NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_not_in_invalid",
            Box::new(|| (Box::new(Fixed32NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_lt_valid",
            Box::new(|| (Box::new(Fixed32Lt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_lt_invalid_equal",
            Box::new(|| (Box::new(Fixed32Lt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_lt_invalid",
            Box::new(|| (Box::new(Fixed32Lt { val: 6 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_lte_valid",
            Box::new(|| (Box::new(Fixed32Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_lte_valid_equal",
            Box::new(|| (Box::new(Fixed32Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_lte_invalid",
            Box::new(|| (Box::new(Fixed32Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gt_valid",
            Box::new(|| (Box::new(Fixed32Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_gt_invalid_equal",
            Box::new(|| (Box::new(Fixed32Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gt_invalid",
            Box::new(|| (Box::new(Fixed32Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gte_valid",
            Box::new(|| (Box::new(Fixed32Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_gte_valid_equal",
            Box::new(|| (Box::new(Fixed32Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_gte_invalid",
            Box::new(|| (Box::new(Fixed32Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gt_lt_valid",
            Box::new(|| (Box::new(Fixed32Gtlt { val: 7 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_gt_lt_invalid_above",
            Box::new(|| (Box::new(Fixed32Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gt_lt_invalid_below",
            Box::new(|| (Box::new(Fixed32Gtlt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gt_lt_invalid_max",
            Box::new(|| (Box::new(Fixed32Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gt_lt_invalid_min",
            Box::new(|| (Box::new(Fixed32Gtlt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(Fixed32ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(Fixed32ExLtgt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed32_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(Fixed32ExLtgt { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(Fixed32ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(Fixed32ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed32_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(Fixed32Gtelte { val: 200 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(Fixed32Gtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(Fixed32Gtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(Fixed32Gtelte { val: 300 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "fixed32_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(Fixed32Gtelte { val: 100 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "fixed32_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(Fixed32ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(Fixed32ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(Fixed32ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(Fixed32ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed32_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(Fixed32ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "fixed32_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(Fixed32Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_none_valid",
            Box::new(|| (Box::new(Fixed64None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_const_valid",
            Box::new(|| (Box::new(Fixed64Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_const_invalid",
            Box::new(|| (Box::new(Fixed64Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_in_valid",
            Box::new(|| (Box::new(Fixed64In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_in_invalid",
            Box::new(|| (Box::new(Fixed64In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_not_in_valid",
            Box::new(|| (Box::new(Fixed64NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_not_in_invalid",
            Box::new(|| (Box::new(Fixed64NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_lt_valid",
            Box::new(|| (Box::new(Fixed64Lt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_lt_invalid_equal",
            Box::new(|| (Box::new(Fixed64Lt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_lt_invalid",
            Box::new(|| (Box::new(Fixed64Lt { val: 6 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_lte_valid",
            Box::new(|| (Box::new(Fixed64Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_lte_valid_equal",
            Box::new(|| (Box::new(Fixed64Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_lte_invalid",
            Box::new(|| (Box::new(Fixed64Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gt_valid",
            Box::new(|| (Box::new(Fixed64Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_gt_invalid_equal",
            Box::new(|| (Box::new(Fixed64Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gt_invalid",
            Box::new(|| (Box::new(Fixed64Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gte_valid",
            Box::new(|| (Box::new(Fixed64Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_gte_valid_equal",
            Box::new(|| (Box::new(Fixed64Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_gte_invalid",
            Box::new(|| (Box::new(Fixed64Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gt_lt_valid",
            Box::new(|| (Box::new(Fixed64Gtlt { val: 7 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_gt_lt_invalid_above",
            Box::new(|| (Box::new(Fixed64Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gt_lt_invalid_below",
            Box::new(|| (Box::new(Fixed64Gtlt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gt_lt_invalid_max",
            Box::new(|| (Box::new(Fixed64Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gt_lt_invalid_min",
            Box::new(|| (Box::new(Fixed64Gtlt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_exclusive_gt_lt_valid_above",
            Box::new(|| (Box::new(Fixed64ExLtgt { val: 11 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_exclusive_gt_lt_valid_below",
            Box::new(|| (Box::new(Fixed64ExLtgt { val: 4 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "fixed64_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(Fixed64ExLtgt { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_exclusive_gt_lt_invalid_max",
            Box::new(|| (Box::new(Fixed64ExLtgt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(Fixed64ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "fixed64_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(Fixed64Gtelte { val: 200 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(Fixed64Gtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(Fixed64Gtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(Fixed64Gtelte { val: 300 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "fixed64_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(Fixed64Gtelte { val: 100 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "fixed64_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(Fixed64ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(Fixed64ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(Fixed64ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(Fixed64ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "fixed64_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(Fixed64ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "fixed64_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(Fixed64Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_none_valid",
            Box::new(|| (Box::new(SFixed32None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_const_valid",
            Box::new(|| (Box::new(SFixed32Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_const_invalid",
            Box::new(|| (Box::new(SFixed32Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_in_valid",
            Box::new(|| (Box::new(SFixed32In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_in_invalid",
            Box::new(|| (Box::new(SFixed32In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_not_in_valid",
            Box::new(|| (Box::new(SFixed32NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_not_in_invalid",
            Box::new(|| (Box::new(SFixed32NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_lt_valid",
            Box::new(|| (Box::new(SFixed32Lt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_lt_invalid_equal",
            Box::new(|| (Box::new(SFixed32Lt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_lt_invalid",
            Box::new(|| (Box::new(SFixed32Lt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_lte_valid",
            Box::new(|| (Box::new(SFixed32Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_lte_valid_equal",
            Box::new(|| (Box::new(SFixed32Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_lte_invalid",
            Box::new(|| (Box::new(SFixed32Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gt_valid",
            Box::new(|| (Box::new(SFixed32Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_gt_invalid_equal",
            Box::new(|| (Box::new(SFixed32Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gt_invalid",
            Box::new(|| (Box::new(SFixed32Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gte_valid",
            Box::new(|| (Box::new(SFixed32Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_gte_valid_equal",
            Box::new(|| (Box::new(SFixed32Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_gte_invalid",
            Box::new(|| (Box::new(SFixed32Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gt_lt_valid",
            Box::new(|| (Box::new(SFixed32Gtlt { val: 5 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed32_gt_lt_invalid_above",
            Box::new(|| (Box::new(SFixed32Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gt_lt_invalid_below",
            Box::new(|| (Box::new(SFixed32Gtlt { val: -1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gt_lt_invalid_max",
            Box::new(|| (Box::new(SFixed32Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gt_lt_invalid_min",
            Box::new(|| (Box::new(SFixed32Gtlt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_exclusive_gt_lt_valid_above",
            Box::new(|| {
                (
                    Box::new(SFixed32ExLtgt { val: 11 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gt_lt_valid_below",
            Box::new(|| {
                (
                    Box::new(SFixed32ExLtgt { val: -1 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(SFixed32ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_exclusive_gt_lt_invalid_max",
            Box::new(|| {
                (
                    Box::new(SFixed32ExLtgt { val: 10 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(SFixed32ExLtgt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed32_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(SFixed32Gtelte { val: 200 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(SFixed32Gtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(SFixed32Gtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(SFixed32Gtelte { val: 300 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed32_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(SFixed32Gtelte { val: 100 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(SFixed32ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(SFixed32ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(SFixed32ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(SFixed32ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed32_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(SFixed32ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed32_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(SFixed32Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_none_valid",
            Box::new(|| (Box::new(SFixed64None { val: 123 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_const_valid",
            Box::new(|| (Box::new(SFixed64Const { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_const_invalid",
            Box::new(|| (Box::new(SFixed64Const { val: 2 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_in_valid",
            Box::new(|| (Box::new(SFixed64In { val: 3 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_in_invalid",
            Box::new(|| (Box::new(SFixed64In { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_not_in_valid",
            Box::new(|| (Box::new(SFixed64NotIn { val: 1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_not_in_invalid",
            Box::new(|| (Box::new(SFixed64NotIn { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_lt_valid",
            Box::new(|| (Box::new(SFixed64Lt { val: -1 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_lt_invalid_equal",
            Box::new(|| (Box::new(SFixed64Lt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_lt_invalid",
            Box::new(|| (Box::new(SFixed64Lt { val: 1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_lte_valid",
            Box::new(|| (Box::new(SFixed64Lte { val: 63 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_lte_valid_equal",
            Box::new(|| (Box::new(SFixed64Lte { val: 64 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_lte_invalid",
            Box::new(|| (Box::new(SFixed64Lte { val: 65 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gt_valid",
            Box::new(|| (Box::new(SFixed64Gt { val: 17 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_gt_invalid_equal",
            Box::new(|| (Box::new(SFixed64Gt { val: 16 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gt_invalid",
            Box::new(|| (Box::new(SFixed64Gt { val: 15 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gte_valid",
            Box::new(|| (Box::new(SFixed64Gte { val: 9 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_gte_valid_equal",
            Box::new(|| (Box::new(SFixed64Gte { val: 8 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_gte_invalid",
            Box::new(|| (Box::new(SFixed64Gte { val: 7 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gt_lt_valid",
            Box::new(|| (Box::new(SFixed64Gtlt { val: 5 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "sfixed64_gt_lt_invalid_above",
            Box::new(|| (Box::new(SFixed64Gtlt { val: 11 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gt_lt_invalid_below",
            Box::new(|| (Box::new(SFixed64Gtlt { val: -1 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gt_lt_invalid_max",
            Box::new(|| (Box::new(SFixed64Gtlt { val: 10 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gt_lt_invalid_min",
            Box::new(|| (Box::new(SFixed64Gtlt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_exclusive_gt_lt_valid_above",
            Box::new(|| {
                (
                    Box::new(SFixed64ExLtgt { val: 11 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gt_lt_valid_below",
            Box::new(|| {
                (
                    Box::new(SFixed64ExLtgt { val: -1 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gt_lt_invalid",
            Box::new(|| (Box::new(SFixed64ExLtgt { val: 5 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_exclusive_gt_lt_invalid_max",
            Box::new(|| {
                (
                    Box::new(SFixed64ExLtgt { val: 10 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gt_lt_invalid_min",
            Box::new(|| (Box::new(SFixed64ExLtgt { val: 0 }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "sfixed64_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(SFixed64Gtelte { val: 200 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(SFixed64Gtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(SFixed64Gtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(SFixed64Gtelte { val: 300 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed64_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(SFixed64Gtelte { val: 100 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(SFixed64ExGtelte { val: 300 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(SFixed64ExGtelte { val: 100 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(SFixed64ExGtelte { val: 256 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(SFixed64ExGtelte { val: 128 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "sfixed64_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(SFixed64ExGtelte { val: 200 }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "sfixed64_ignore_empty_gte_lte_valid",
            Box::new(|| (Box::new(SFixed64Ignore { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "bool_none_valid",
            Box::new(|| (Box::new(BoolNone { val: true }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "bool_const_true_valid",
            Box::new(|| {
                (
                    Box::new(BoolConstTrue { val: true }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bool_const_true_invalid",
            Box::new(|| {
                (
                    Box::new(BoolConstTrue { val: false }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bool_const_false_valid",
            Box::new(|| {
                (
                    Box::new(BoolConstFalse { val: false }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bool_const_false_invalid",
            Box::new(|| {
                (
                    Box::new(BoolConstFalse { val: true }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_none_valid",
            Box::new(|| {
                (
                    Box::new(StringNone {
                        val: "quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_const_valid",
            Box::new(|| {
                (
                    Box::new(StringConst {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_const_invalid",
            Box::new(|| {
                (
                    Box::new(StringConst {
                        val: "bar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_in_valid",
            Box::new(|| {
                (
                    Box::new(StringIn {
                        val: "bar".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_in_invalid",
            Box::new(|| {
                (
                    Box::new(StringIn {
                        val: "quux".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_not_in_valid",
            Box::new(|| {
                (
                    Box::new(StringNotIn {
                        val: "quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(StringNotIn {
                        val: "fizz".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_len_valid",
            Box::new(|| {
                (
                    Box::new(StringLen {
                        val: "baz".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_len_valid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringLen {
                        val: "你好吖".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_len_invalid_lt",
            Box::new(|| {
                (
                    Box::new(StringLen {
                        val: "go".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_len_invalid_gt",
            Box::new(|| {
                (
                    Box::new(StringLen {
                        val: "fizz".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_len_invalid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringLen {
                        val: "你好".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_len_valid",
            Box::new(|| {
                (
                    Box::new(StringMinLen {
                        val: "protoc".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_len_valid_min",
            Box::new(|| {
                (
                    Box::new(StringMinLen {
                        val: "baz".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_len_invalid",
            Box::new(|| {
                (
                    Box::new(StringMinLen {
                        val: "go".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_len_invalid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringMinLen {
                        val: "你好".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_max_len_valid",
            Box::new(|| {
                (
                    Box::new(StringMaxLen {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_max_len_valid_max",
            Box::new(|| {
                (
                    Box::new(StringMaxLen {
                        val: "proto".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_max_len_valid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringMaxLen {
                        val: "你好你好".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_max_len_invalid",
            Box::new(|| {
                (
                    Box::new(StringMaxLen {
                        val: "1234567890".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_max_len_valid",
            Box::new(|| {
                (
                    Box::new(StringMinMaxLen {
                        val: "quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_len_valid_min",
            Box::new(|| {
                (
                    Box::new(StringMinMaxLen {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_len_valid_max",
            Box::new(|| {
                (
                    Box::new(StringMinMaxLen {
                        val: "proto".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_len_valid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringMinMaxLen {
                        val: "你好你好".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_len_invalid_below",
            Box::new(|| {
                (
                    Box::new(StringMinMaxLen {
                        val: "go".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_max_len_invalid_above",
            Box::new(|| {
                (
                    Box::new(StringMinMaxLen {
                        val: "validate".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_equal_min_max_len_valid",
            Box::new(|| {
                (
                    Box::new(StringEqualMinMaxLen {
                        val: "proto".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_equal_min_max_len_invalid",
            Box::new(|| {
                (
                    Box::new(StringEqualMinMaxLen {
                        val: "validate".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_len_bytes_valid",
            Box::new(|| {
                (
                    Box::new(StringLenBytes {
                        val: "pace".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_len_bytes_invalid_lt",
            Box::new(|| {
                (
                    Box::new(StringLenBytes {
                        val: "val".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_len_bytes_invalid_gt",
            Box::new(|| {
                (
                    Box::new(StringLenBytes {
                        val: "world".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_len_bytes_invalid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringLenBytes {
                        val: "世界和平".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_bytes_valid",
            Box::new(|| {
                (
                    Box::new(StringMinBytes {
                        val: "proto".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_bytes_valid_min",
            Box::new(|| {
                (
                    Box::new(StringMinBytes {
                        val: "quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_bytes_valid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringMinBytes {
                        val: "你好".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_bytes_invalid",
            Box::new(|| {
                (
                    Box::new(StringMinBytes {
                        val: "".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_max_bytes_valid",
            Box::new(|| {
                (
                    Box::new(StringMaxBytes {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_max_bytes_valid_max",
            Box::new(|| {
                (
                    Box::new(StringMaxBytes {
                        val: "12345678".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_max_bytes_invalid",
            Box::new(|| {
                (
                    Box::new(StringMaxBytes {
                        val: "123456789".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_max_bytes_invalid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringMaxBytes {
                        val: "你好你好你好".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_max_bytes_valid",
            Box::new(|| {
                (
                    Box::new(StringMinMaxBytes {
                        val: "protoc".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_bytes_valid_min",
            Box::new(|| {
                (
                    Box::new(StringMinMaxBytes {
                        val: "quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_bytes_valid_max",
            Box::new(|| {
                (
                    Box::new(StringMinMaxBytes {
                        val: "fizzbuzz".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_bytes_valid_multibyte",
            Box::new(|| {
                (
                    Box::new(StringMinMaxBytes {
                        val: "你好".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_min_max_bytes_invalid_below",
            Box::new(|| {
                (
                    Box::new(StringMinMaxBytes {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_min_max_bytes_invalid_above",
            Box::new(|| {
                (
                    Box::new(StringMinMaxBytes {
                        val: "你好你好你".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_equal_min_max_bytes_valid",
            Box::new(|| {
                (
                    Box::new(StringEqualMinMaxBytes {
                        val: "protoc".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_equal_min_max_bytes_invalid",
            Box::new(|| {
                (
                    Box::new(StringEqualMinMaxBytes {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_pattern_valid",
            Box::new(|| {
                (
                    Box::new(StringPattern {
                        val: "Foo123".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_pattern_invalid",
            Box::new(|| {
                (
                    Box::new(StringPattern {
                        val: "!@#$%^&*()".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_pattern_invalid_empty",
            Box::new(|| {
                (
                    Box::new(StringPattern {
                        val: "".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_pattern_invalid_null",
            Box::new(|| {
                (
                    Box::new(StringPattern {
                        val: "a\x0000".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_pattern_escapes_valid",
            Box::new(|| {
                (
                    Box::new(StringPatternEscapes {
                        val: "* \\ x".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_pattern_escapes_invalid",
            Box::new(|| {
                (
                    Box::new(StringPatternEscapes {
                        val: "invalid".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_pattern_escapes_invalid_empty",
            Box::new(|| {
                (
                    Box::new(StringPatternEscapes {
                        val: "".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_prefix_valid",
            Box::new(|| {
                (
                    Box::new(StringPrefix {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_prefix_valid_only",
            Box::new(|| {
                (
                    Box::new(StringPrefix {
                        val: "foo".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_prefix_invalid",
            Box::new(|| {
                (
                    Box::new(StringPrefix {
                        val: "bar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_prefix_invalid_case_sensitive",
            Box::new(|| {
                (
                    Box::new(StringPrefix {
                        val: "Foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_contains_valid",
            Box::new(|| {
                (
                    Box::new(StringContains {
                        val: "candy bars".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_contains_valid_only",
            Box::new(|| {
                (
                    Box::new(StringContains {
                        val: "bar".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_contains_invalid",
            Box::new(|| {
                (
                    Box::new(StringContains {
                        val: "candy bazs".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_contains_invalid_case_sensitive",
            Box::new(|| {
                (
                    Box::new(StringContains {
                        val: "Candy Bars".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_not_contains_valid",
            Box::new(|| {
                (
                    Box::new(StringNotContains {
                        val: "candy bazs".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_not_contains_valid_case_sensitive",
            Box::new(|| {
                (
                    Box::new(StringNotContains {
                        val: "Candy Bars".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_not_contains_invalid",
            Box::new(|| {
                (
                    Box::new(StringNotContains {
                        val: "candy bars".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_not_contains_invalid_equal",
            Box::new(|| {
                (
                    Box::new(StringNotContains {
                        val: "bar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_suffix_valid",
            Box::new(|| {
                (
                    Box::new(StringSuffix {
                        val: "foobaz".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_suffix_valid_only",
            Box::new(|| {
                (
                    Box::new(StringSuffix {
                        val: "baz".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_suffix_invalid",
            Box::new(|| {
                (
                    Box::new(StringSuffix {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_suffix_invalid_case_sensitive",
            Box::new(|| {
                (
                    Box::new(StringSuffix {
                        val: "FooBaz".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_email_valid",
            Box::new(|| {
                (
                    Box::new(StringEmail {
                        val: "foo@bar.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_email_valid_name",
            Box::new(|| {
                (
                    Box::new(StringEmail {
                        val: "John Smith <foo@bar.com>".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_email_invalid",
            Box::new(|| {
                (
                    Box::new(StringEmail {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_email_invalid_local_segment_too_long",
            Box::new(|| {
                (Box::new(StringEmail{val: "x0123456789012345678901234567890123456789012345678901234567890123456789@example.com".to_string()}) as Box<dyn Validator>, 1)
            }) as Factory,
        ),
        (
            "string_email_invalid_hostname_too_long",
            Box::new(|| {
                (Box::new(StringEmail{val: "foo@x0123456789012345678901234567890123456789012345678901234567890123456789.com".to_string()}) as Box<dyn Validator>, 1)
            }) as Factory,
        ),
        (
            "string_email_invalid_bad_hostname",
            Box::new(|| {
                (
                    Box::new(StringEmail {
                        val: "foo@-bar.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_email_empty",
            Box::new(|| {
                (
                    Box::new(StringEmail {
                        val: "".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_valid_hostname",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "example.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_address_valid_hostname_uppercase",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "ASD.example.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_address_valid_hostname_hyphens",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "foo-bar.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_address_valid_hostname_trailing_dot",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "example.com.".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "!@#$%^&".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname_underscore",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "foo_bar.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname_too_long",
            Box::new(|| {
                (Box::new(StringAddress{val: "x0123456789012345678901234567890123456789012345678901234567890123456789.com".to_string()}) as Box<dyn Validator>, 1)
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname_trailing_hyphens",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "foo-bar-.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname_leading_hyphens",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "foo-bar.-com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname_empty",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "asd..asd.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_hostname_idns",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "你好.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_address_valid_ip_v4",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "192.168.0.1".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_address_valid_ip_v6",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "3e::99".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_address_invalid_ip",
            Box::new(|| {
                (
                    Box::new(StringAddress {
                        val: "ff::fff::0b".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_hostname_valid",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "example.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_hostname_valid_uppercase",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "ASD.example.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_hostname_valid_hyphens",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "foo-bar.com".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_hostname_valid_trailing_dot",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "example.com.".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_hostname_invalid",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "!@#$%^&".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_hostname_invalid_underscore",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "foo_bar.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_hostname_invalid_too_long",
            Box::new(|| {
                (Box::new(StringHostname{val: "x0123456789012345678901234567890123456789012345678901234567890123456789.com".to_string()}) as Box<dyn Validator>, 1)
            }) as Factory,
        ),
        (
            "string_hostname_invalid_trailing_hyphens",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "foo-bar-.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_hostname_invalid_leading_hyphens",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "foo-bar.-com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_hostname_invalid_empty",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "asd..asd.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_hostname_invalid_idns",
            Box::new(|| {
                (
                    Box::new(StringHostname {
                        val: "你好.com".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ip_valid_v4",
            Box::new(|| {
                (
                    Box::new(StringIp {
                        val: "192.168.0.1".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_ip_valid_v6",
            Box::new(|| {
                (
                    Box::new(StringIp {
                        val: "3e::99".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_ip_invalid",
            Box::new(|| {
                (
                    Box::new(StringIp {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ipv4_valid",
            Box::new(|| {
                (
                    Box::new(StringIPv4 {
                        val: "192.168.0.1".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_ipv4_invalid",
            Box::new(|| {
                (
                    Box::new(StringIPv4 {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ipv4_invalid_erroneous",
            Box::new(|| {
                (
                    Box::new(StringIPv4 {
                        val: "256.0.0.0".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ipv4_invalid_v6",
            Box::new(|| {
                (
                    Box::new(StringIPv4 {
                        val: "3e::99".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ipv6_valid",
            Box::new(|| {
                (
                    Box::new(StringIPv6 {
                        val: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_ipv6_valid_collapsed",
            Box::new(|| {
                (
                    Box::new(StringIPv6 {
                        val: "2001:db8:85a3::8a2e:370:7334".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_ipv6_invalid",
            Box::new(|| {
                (
                    Box::new(StringIPv6 {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ipv6_invalid_v4",
            Box::new(|| {
                (
                    Box::new(StringIPv6 {
                        val: "192.168.0.1".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_ipv6_invalid_erroneous",
            Box::new(|| {
                (
                    Box::new(StringIPv6 {
                        val: "ff::fff::0b".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_uri_valid_2",
            Box::new(|| {
                (
                    Box::new(StringUri {
                        val: "http://example.com/foo/bar?baz=quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uri_invalid_2",
            Box::new(|| {
                (
                    Box::new(StringUri {
                        val: "!@#$%^&*%$#".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_uri_invalid_relative",
            Box::new(|| {
                (
                    Box::new(StringUri {
                        val: "/foo/bar?baz=quux".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_uri_valid_3",
            Box::new(|| {
                (
                    Box::new(StringUriRef {
                        val: "http://example.com/foo/bar?baz=quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uri_valid_relative",
            Box::new(|| {
                (
                    Box::new(StringUriRef {
                        val: "/foo/bar?baz=quux".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uri_invalid_3",
            Box::new(|| {
                (
                    Box::new(StringUriRef {
                        val: "!@#$%^&*%$#".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_nil",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "00000000-0000-0000-0000-000000000000".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v1",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "b45c0c80-8880-11e9-a5b1-000000000000".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v1_case_insensitive",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "B45C0C80-8880-11E9-A5B1-000000000000".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v2",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "b45c0c80-8880-21e9-a5b1-000000000000".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v2_case_insensitive",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "B45C0C80-8880-21E9-A5B1-000000000000".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v3",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "a3bb189e-8bf9-3888-9912-ace4e6543002".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v3_case_insensitive",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "A3BB189E-8BF9-3888-9912-ACE4E6543002".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v4",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "8b208305-00e8-4460-a440-5e0dcd83bb0a".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v4_case_insensitive",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "8B208305-00E8-4460-A440-5E0DCD83BB0A".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v5",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "a6edc906-2f9f-5fb2-a373-efac406f0ef2".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_v5_case_insensitive",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "A6EDC906-2F9F-5FB2-A373-EFAC406F0EF2".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_uuid_invalid",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "foobar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_uuid_invalid_bad_uuid",
            Box::new(|| {
                (
                    Box::new(StringUuid {
                        val: "ffffffff-ffff-ffff-ffff-fffffffffffff".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_uuid_valid_ignore_empty",
            Box::new(|| {
                (
                    Box::new(StringUuidIgnore {
                        val: "".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_valid",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "clustername".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_valid_2",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: ":path".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_valid_nums",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "cluster-123".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_valid_special_token",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "!+#&.%".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_valid_period",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "CLUSTER.NAME".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_invalid",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: ":".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_invalid_2",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: ":path:".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_invalid_space",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "cluster name".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_invalid_return",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "example\r".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_invalid_tab",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "example\t".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_name_invalid_slash",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderName {
                        val: "/test/long/url".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_valid",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "cluster.name.123".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_valid_uppercase",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "/TEST/LONG/URL".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_valid_spaces",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "cluster name".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_valid_tab",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "example\t".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_valid_special_token",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "!#%&./+".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_invalid_nul",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "foo\u{0000}bar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_invalid_del",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "\u{007f}".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_http_header_value_invalid",
            Box::new(|| {
                (
                    Box::new(StringHttpHeaderValue {
                        val: "example\r".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_valid",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "cluster.name.123".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_valid_uppercase",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "/TEST/LONG/URL".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_valid_spaces",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "cluster name".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_valid_tab",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "example\t".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_valid_del",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "\u{007f}".to_string(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_invalid_nul",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "foo\u{0000}bar".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_invalid_cr",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "example\r".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "string_non_strict_valid_header_invalid_nl",
            Box::new(|| {
                (
                    Box::new(StringValidHeader {
                        val: "exa\u{000A}mple".to_string(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_none_valid",
            Box::new(|| {
                (
                    Box::new(BytesNone {
                        val: b"quux".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_const_valid",
            Box::new(|| {
                (
                    Box::new(BytesConst {
                        val: b"foo".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_const_invalid",
            Box::new(|| {
                (
                    Box::new(BytesConst {
                        val: b"bar".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_in_valid",
            Box::new(|| {
                (
                    Box::new(BytesIn {
                        val: b"bar".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_in_invalid",
            Box::new(|| {
                (
                    Box::new(BytesIn {
                        val: b"quux".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_not_in_valid",
            Box::new(|| {
                (
                    Box::new(BytesNotIn {
                        val: b"quux".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(BytesNotIn {
                        val: b"fizz".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_len_valid",
            Box::new(|| {
                (
                    Box::new(BytesLen {
                        val: b"baz".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_len_invalid_lt",
            Box::new(|| {
                (
                    Box::new(BytesLen {
                        val: b"go".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_len_invalid_gt",
            Box::new(|| {
                (
                    Box::new(BytesLen {
                        val: b"fizz".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_min_len_valid",
            Box::new(|| {
                (
                    Box::new(BytesMinLen {
                        val: b"fizz".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_min_len_valid_min",
            Box::new(|| {
                (
                    Box::new(BytesMinLen {
                        val: b"baz".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_min_len_invalid",
            Box::new(|| {
                (
                    Box::new(BytesMinLen {
                        val: b"go".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_max_len_valid",
            Box::new(|| {
                (
                    Box::new(BytesMaxLen {
                        val: b"foo".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_max_len_valid_max",
            Box::new(|| {
                (
                    Box::new(BytesMaxLen {
                        val: b"proto".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_max_len_invalid",
            Box::new(|| {
                (
                    Box::new(BytesMaxLen {
                        val: b"1234567890".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_min_max_len_valid",
            Box::new(|| {
                (
                    Box::new(BytesMinMaxLen {
                        val: b"quux".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_min_max_len_valid_min",
            Box::new(|| {
                (
                    Box::new(BytesMinMaxLen {
                        val: b"foo".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_min_max_len_valid_max",
            Box::new(|| {
                (
                    Box::new(BytesMinMaxLen {
                        val: b"proto".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_min_max_len_invalid_below",
            Box::new(|| {
                (
                    Box::new(BytesMinMaxLen {
                        val: b"go".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_min_max_len_invalid_above",
            Box::new(|| {
                (
                    Box::new(BytesMinMaxLen {
                        val: b"validate".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_equal_min_max_len_valid",
            Box::new(|| {
                (
                    Box::new(BytesEqualMinMaxLen {
                        val: b"proto".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_equal_min_max_len_invalid",
            Box::new(|| {
                (
                    Box::new(BytesEqualMinMaxLen {
                        val: b"validate".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_pattern_valid",
            Box::new(|| {
                (
                    Box::new(BytesPattern {
                        val: b"Foo123".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        // b"你好你好"
        (
            "bytes_pattern_invalid",
            Box::new(|| {
                (
                    Box::new(BytesPattern {
                        val: b"\xE4\xBD\xA0\xE5\xA5\xBD\xE4\xBD\xA0\xE5\xA5\xBD".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_pattern_invalid_empty",
            Box::new(|| {
                (
                    Box::new(BytesPattern { val: b"".to_vec() }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_prefix_valid",
            Box::new(|| {
                (
                    Box::new(BytesPrefix {
                        val: vec![0x99, 0x9f, 0x08],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_prefix_valid_only",
            Box::new(|| {
                (
                    Box::new(BytesPrefix { val: vec![0x99] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_prefix_invalid",
            Box::new(|| {
                (
                    Box::new(BytesPrefix {
                        val: b"bar".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_contains_valid",
            Box::new(|| {
                (
                    Box::new(BytesContains {
                        val: b"candy bars".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_contains_valid_only",
            Box::new(|| {
                (
                    Box::new(BytesContains {
                        val: b"bar".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_contains_invalid",
            Box::new(|| {
                (
                    Box::new(BytesContains {
                        val: b"candy bazs".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_suffix_valid",
            Box::new(|| {
                (
                    Box::new(BytesSuffix {
                        val: vec![0x62, 0x75, 0x7A, 0x7A],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_suffix_valid_only",
            Box::new(|| {
                (
                    Box::new(BytesSuffix {
                        val: b"\x62\x75\x7A\x7A".to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_suffix_invalid",
            Box::new(|| {
                (
                    Box::new(BytesSuffix {
                        val: b"foobar".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_suffix_invalid_case_sensitive",
            Box::new(|| {
                (
                    Box::new(BytesSuffix {
                        val: b"FooBaz".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_ip_valid_v4",
            Box::new(|| {
                (
                    Box::new(BytesIp {
                        val: vec![0xC0, 0xA8, 0x00, 0x01],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_ip_valid_v6",
            Box::new(|| {
                (
                    Box::new(BytesIp {
                        val: b"\x20\x01\x0D\xB8\x85\xA3\x00\x00\x00\x00\x8A\x2E\x03\x70\x73\x34"
                            .to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_ip_invalid",
            Box::new(|| {
                (
                    Box::new(BytesIp {
                        val: b"foobar".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv4_valid",
            Box::new(|| {
                (
                    Box::new(BytesIPv4 {
                        val: vec![0xC0, 0xA8, 0x00, 0x01],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv4_invalid",
            Box::new(|| {
                (
                    Box::new(BytesIPv4 {
                        val: b"foobar".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv4_invalid_v6",
            Box::new(|| {
                (
                    Box::new(BytesIPv4 {
                        val: b"\x20\x01\x0D\xB8\x85\xA3\x00\x00\x00\x00\x8A\x2E\x03\x70\x73\x34"
                            .to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv6_valid",
            Box::new(|| {
                (
                    Box::new(BytesIPv6 {
                        val: b"\x20\x01\x0D\xB8\x85\xA3\x00\x00\x00\x00\x8A\x2E\x03\x70\x73\x34"
                            .to_vec(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv6_invalid",
            Box::new(|| {
                (
                    Box::new(BytesIPv6 {
                        val: b"fooar".to_vec(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv6_invalid_v4",
            Box::new(|| {
                (
                    Box::new(BytesIPv6 {
                        val: vec![0xC0, 0xA8, 0x00, 0x01],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "bytes_ipv6_valid_ignore_empty",
            Box::new(|| {
                (
                    Box::new(BytesIPv6Ignore::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_none_valid",
            Box::new(|| {
                (
                    Box::new(EnumNone {
                        val: TestEnum::One.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_const_valid",
            Box::new(|| {
                (
                    Box::new(EnumConst {
                        val: TestEnum::Two.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_const_invalid",
            Box::new(|| {
                (
                    Box::new(EnumConst {
                        val: TestEnum::One.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_alias_const_valid",
            Box::new(|| {
                (
                    Box::new(EnumAliasConst {
                        val: TestEnumAlias::C.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_alias_const_valid_alias",
            Box::new(|| {
                (
                    Box::new(EnumAliasConst {
                        val: TestEnumAlias::C.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_alias_const_invalid",
            Box::new(|| {
                (
                    Box::new(EnumAliasConst {
                        val: TestEnumAlias::A.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_defined_only_valid",
            Box::new(|| (Box::new(EnumDefined { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "enum_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(EnumDefined { val: i32::MAX }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_alias_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(EnumAliasDefined { val: 1 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_alias_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(EnumAliasDefined { val: i32::MAX }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_in_valid",
            Box::new(|| {
                (
                    Box::new(EnumIn {
                        val: TestEnum::Two.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_in_invalid",
            Box::new(|| {
                (
                    Box::new(EnumIn {
                        val: TestEnum::One.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_alias_in_valid",
            Box::new(|| {
                (
                    Box::new(EnumAliasIn {
                        val: TestEnumAlias::A.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_alias_in_valid_alias",
            Box::new(|| {
                (
                    Box::new(EnumAliasIn {
                        val: TestEnumAlias::A.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_alias_in_invalid",
            Box::new(|| {
                (
                    Box::new(EnumAliasIn {
                        val: TestEnumAlias::B.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_not_in_valid",
            Box::new(|| {
                (
                    Box::new(EnumNotIn {
                        val: TestEnum::Zero.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_not_in_valid_undefined",
            Box::new(|| {
                (
                    Box::new(EnumNotIn { val: i32::MAX }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(EnumNotIn {
                        val: TestEnum::One.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_alias_not_in_valid",
            Box::new(|| {
                (
                    Box::new(EnumAliasNotIn {
                        val: TestEnumAlias::A.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_alias_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(EnumAliasNotIn {
                        val: TestEnumAlias::B.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_alias_not_in_invalid_alias",
            Box::new(|| {
                (
                    Box::new(EnumAliasNotIn {
                        val: TestEnumAlias::B.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_external_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(EnumExternal {
                        val: other_package::embed::Enumerated::Value.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_external_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(EnumExternal { val: i32::MAX }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_external_in_valid",
            Box::new(|| {
                (
                    Box::new(EnumExternal3 {
                        foo: other_package::embed::FooNumber::Zero.into(),
                        bar: 0,
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_external_in_invalid",
            Box::new(|| {
                (
                    Box::new(EnumExternal3 {
                        foo: other_package::embed::FooNumber::One.into(),
                        bar: 0,
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_external_not_in_valid",
            Box::new(|| {
                (
                    Box::new(EnumExternal3 {
                        bar: yet_another_package::embed::BarNumber::Zero.into(),
                        foo: 0,
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_external_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(EnumExternal3 {
                        bar: yet_another_package::embed::BarNumber::One.into(),
                        foo: 0,
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_external_const_valid",
            Box::new(|| {
                (
                    Box::new(EnumExternal4 {
                        sort_direction: sort::Direction::Asc.into(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_external_const_invalid",
            Box::new(|| {
                (
                    Box::new(EnumExternal4 {
                        sort_direction: sort::Direction::Desc.into(),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumDefined {
                        val: vec![TestEnum::One.into(), TestEnum::Two.into()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumDefined {
                        val: vec![TestEnum::One.into(), i32::MAX],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_external_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedExternalEnumDefined {
                        val: vec![other_package::embed::Enumerated::Value.into()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_external_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedExternalEnumDefined {
                        val: vec![i32::MAX],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_another_external_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedYetAnotherExternalEnumDefined {
                        val: vec![yet_another_package::embed::Enumerated::Value.into()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_external_in_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumExternal {
                        foo: vec![
                            other_package::embed::FooNumber::Zero.into(),
                            other_package::embed::FooNumber::Two.into(),
                        ],
                        ..RepeatedEnumExternal::default()
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_external_in_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumExternal {
                        foo: vec![other_package::embed::FooNumber::One.into()],
                        ..RepeatedEnumExternal::default()
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_external_not_in_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumExternal {
                        bar: vec![
                            yet_another_package::embed::BarNumber::Zero.into(),
                            yet_another_package::embed::BarNumber::Two.into(),
                        ],
                        ..RepeatedEnumExternal::default()
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_repeated_external_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumExternal {
                        bar: vec![yet_another_package::embed::BarNumber::One.into()],
                        ..RepeatedEnumExternal::default()
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_map_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(MapEnumDefined {
                        val: HashMap::from([("foo".to_string(), TestEnum::Two.into())]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_map_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(MapEnumDefined {
                        val: HashMap::from([("foo".to_string(), i32::MAX)]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "enum_map_external_defined_only_valid",
            Box::new(|| {
                (
                    Box::new(MapExternalEnumDefined {
                        val: HashMap::from([(
                            "foo".to_string(),
                            other_package::embed::Enumerated::Value.into(),
                        )]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "enum_map_external_defined_only_invalid",
            Box::new(|| {
                (
                    Box::new(MapExternalEnumDefined {
                        val: HashMap::from([("foo".to_string(), i32::MAX)]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_none_valid",
            Box::new(|| {
                (
                    Box::new(MessageNone {
                        val: Some(message_none::NoneMsg::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_none_valid_unset",
            Box::new(|| (Box::new(MessageNone::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "message_disabled_valid",
            Box::new(|| {
                (
                    Box::new(MessageDisabled { val: 456 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_disabled_valid_invalid_field",
            Box::new(|| {
                (
                    Box::new(MessageDisabled { val: 0 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_ignored_valid",
            Box::new(|| {
                (
                    Box::new(MessageIgnored { val: 456 }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_ignored_valid_invalid_field",
            Box::new(|| (Box::new(MessageIgnored { val: 0 }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "message_field_valid",
            Box::new(|| {
                (
                    Box::new(cases::Message {
                        val: Some(TestMsg {
                            r#const: "foo".to_string(),
                            ..TestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_field_valid_unset",
            Box::new(|| (Box::new(cases::Message::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "message_field_invalid",
            Box::new(|| {
                (
                    Box::new(cases::Message {
                        val: Some(TestMsg::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_field_invalid_transitive",
            Box::new(|| {
                (
                    Box::new(cases::Message {
                        val: Some(TestMsg {
                            r#const: "foo".to_string(),
                            nested: Some(Box::new(TestMsg::default())),
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_skip_valid",
            Box::new(|| {
                (
                    Box::new(MessageSkip {
                        val: Some(TestMsg::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_required_valid",
            Box::new(|| {
                (
                    Box::new(MessageRequired {
                        val: Some(TestMsg {
                            r#const: "foo".to_string(),
                            ..TestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_required_valid_oneof",
            Box::new(|| {
                (
                    Box::new(MessageRequiredOneof {
                        one: Some(One::Val(TestMsg {
                            r#const: "foo".to_string(),
                            ..TestMsg::default()
                        })),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_required_invalid",
            Box::new(|| {
                (
                    Box::new(MessageRequired::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_required_invalid_oneof",
            Box::new(|| {
                (
                    Box::new(MessageRequiredOneof::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_cross_package_embed_none_valid",
            Box::new(|| {
                (
                    Box::new(MessageCrossPackage {
                        val: Some(other_package::Embed { val: 1 }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_cross_package_embed_none_valid_nil",
            Box::new(|| {
                (
                    Box::new(MessageCrossPackage::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_cross_package_embed_none_valid_empty",
            Box::new(|| {
                (
                    Box::new(MessageCrossPackage {
                        val: Some(other_package::Embed::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_cross_package_embed_none_invalid",
            Box::new(|| {
                (
                    Box::new(MessageCrossPackage {
                        val: Some(other_package::Embed { val: -1 }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "message_required_valid_2",
            Box::new(|| {
                (
                    Box::new(MessageRequiredButOptional {
                        val: Some(TestMsg {
                            r#const: "foo".to_string(),
                            ..TestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "message_required_valid_unset",
            Box::new(|| {
                (
                    Box::new(MessageRequiredButOptional::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_none_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedNone { val: vec![1, 2, 3] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_embed_none_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedNone {
                        val: vec![cases::Embed { val: 1 }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_embed_none_valid_nil",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedNone::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_embed_none_valid_empty",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedNone::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_embed_none_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedNone {
                        val: vec![cases::Embed { val: -1 }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_cross_package_embed_none_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedCrossPackageNone {
                        val: vec![other_package::Embed { val: 1 }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_cross_package_embed_none_valid_nil",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedCrossPackageNone::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_cross_package_embed_none_valid_empty",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedCrossPackageNone::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_cross_package_embed_none_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedCrossPackageNone {
                        val: vec![other_package::Embed { val: -1 }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_min_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedMin {
                        val: vec![
                            cases::Embed { val: 1 },
                            cases::Embed { val: 2 },
                            cases::Embed { val: 3 },
                        ],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_valid_equal",
            Box::new(|| {
                (
                    Box::new(RepeatedMin {
                        val: vec![cases::Embed { val: 1 }, cases::Embed { val: 2 }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedMin {
                        val: vec![cases::Embed { val: 1 }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_min_invalid_element",
            Box::new(|| {
                (
                    Box::new(RepeatedMin {
                        val: vec![cases::Embed { val: 1 }, cases::Embed { val: -1 }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_max_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedMax { val: vec![1., 2.] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_max_valid_equal",
            Box::new(|| {
                (
                    Box::new(RepeatedMax {
                        val: vec![1., 2., 3.],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_max_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedMax {
                        val: vec![1., 2., 3., 4.],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_min_max_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedMinMax { val: vec![1, 2, 3] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_max_valid_min",
            Box::new(|| {
                (
                    Box::new(RepeatedMinMax { val: vec![1, 2] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_max_valid_max",
            Box::new(|| {
                (
                    Box::new(RepeatedMinMax {
                        val: vec![1, 2, 3, 4],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_max_invalid_below",
            Box::new(|| (Box::new(RepeatedMinMax::default()) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "repeated_min_max_invalid_above",
            Box::new(|| {
                (
                    Box::new(RepeatedMinMax {
                        val: vec![1, 2, 3, 4, 5],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_exact_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedExact { val: vec![1, 2, 3] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_exact_invalid_below",
            Box::new(|| {
                (
                    Box::new(RepeatedExact { val: vec![1, 2] }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_exact_invalid_above",
            Box::new(|| {
                (
                    Box::new(RepeatedExact {
                        val: vec![1, 2, 3, 4],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_unique_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedUnique {
                        val: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_unique_valid_empty",
            Box::new(|| (Box::new(RepeatedUnique::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "repeated_unique_valid_case_sensitivity",
            Box::new(|| {
                (
                    Box::new(RepeatedUnique {
                        val: vec!["foo".to_string(), "Foo".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_unique_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedUnique {
                        val: vec![
                            "foo".to_string(),
                            "bar".to_string(),
                            "foo".to_string(),
                            "baz".to_string(),
                        ],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedItemRule {
                        val: vec![1., 2., 3.],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_empty",
            Box::new(|| {
                (
                    Box::new(RepeatedItemRule::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_pattern",
            Box::new(|| {
                (
                    Box::new(RepeatedItemPattern {
                        val: vec!["Alpha".to_string(), "Beta123".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedItemRule {
                        val: vec![1., -2., 3.],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_pattern",
            Box::new(|| {
                (
                    Box::new(RepeatedItemPattern {
                        val: vec!["Alpha".to_string(), "!@#$%^&*()".to_string()],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_in",
            Box::new(|| {
                (
                    Box::new(RepeatedItemIn {
                        val: vec!["baz".to_string()],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_in",
            Box::new(|| {
                (
                    Box::new(RepeatedItemIn {
                        val: vec!["foo".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedItemNotIn {
                        val: vec!["foo".to_string()],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedItemNotIn {
                        val: vec!["baz".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_enum_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumIn { val: vec![1] }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_enum_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumIn { val: vec![0] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_enum_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumNotIn { val: vec![0] }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_enum_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEnumNotIn { val: vec![1] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_embedded_enum_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbeddedEnumIn { val: vec![1] }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_embedded_enum_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbeddedEnumIn { val: vec![0] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_embedded_enum_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbeddedEnumNotIn { val: vec![0] }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_embedded_enum_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbeddedEnumNotIn { val: vec![1] }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_any_in",
            Box::new(|| {
                (
                    Box::new(RepeatedAnyIn {
                        val: vec![Any {
                            type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(),
                            ..Any::default()
                        }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_any_in",
            Box::new(|| {
                (
                    Box::new(RepeatedAnyIn {
                        val: vec![Any {
                            type_url: "type.googleapis.com/google.protobuf.Duration".to_string(),
                            ..Any::default()
                        }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_items_invalid_any_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedAnyNotIn {
                        val: vec![Any {
                            type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(),
                            ..Any::default()
                        }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_items_valid_any_not_in",
            Box::new(|| {
                (
                    Box::new(RepeatedAnyNotIn {
                        val: vec![Any {
                            type_url: "type.googleapis.com/google.protobuf.Duration".to_string(),
                            ..Any::default()
                        }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_embed_skip_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedSkip {
                        val: vec![cases::Embed { val: 1 }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_embed_skip_valid_invalid_element",
            Box::new(|| {
                (
                    Box::new(RepeatedEmbedSkip {
                        val: vec![cases::Embed { val: -1 }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_and_items_len_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedMinAndItemLen {
                        val: vec!["aaa".to_string(), "bbb".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_and_items_len_invalid_min",
            Box::new(|| {
                (
                    Box::new(RepeatedMinAndItemLen {
                        val: vec![String::default()],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_min_and_items_len_invalid_len",
            Box::new(|| {
                (
                    Box::new(RepeatedMinAndItemLen {
                        val: vec!["x".to_string()],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_min_and_max_items_len_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedMinAndMaxItemLen {
                        val: vec!["aaa".to_string(), "bbb".to_string()],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_min_and_max_items_len_invalid_min_len",
            Box::new(|| {
                (
                    Box::new(RepeatedMinAndMaxItemLen::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_min_and_max_items_len_invalid_max_len",
            Box::new(|| {
                (
                    Box::new(RepeatedMinAndMaxItemLen {
                        val: vec![
                            "aaa".to_string(),
                            "bbb".to_string(),
                            "ccc".to_string(),
                            "ddd".to_string(),
                        ],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_duration_gte_valid",
            Box::new(|| {
                (
                    Box::new(RepeatedDuration {
                        val: vec![Duration {
                            seconds: 3,
                            nanos: 0,
                        }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_duration_gte_valid_empty",
            Box::new(|| {
                (
                    Box::new(RepeatedDuration::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_duration_gte_valid_equal",
            Box::new(|| {
                (
                    Box::new(RepeatedDuration {
                        val: vec![Duration {
                            nanos: 1000000,
                            seconds: 0,
                        }],
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "repeated_duration_gte_invalid",
            Box::new(|| {
                (
                    Box::new(RepeatedDuration {
                        val: vec![Duration {
                            seconds: -1,
                            nanos: 0,
                        }],
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "repeated_exact_valid_ignore_empty",
            Box::new(|| {
                (
                    Box::new(RepeatedExactIgnore::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_none_valid",
            Box::new(|| {
                (
                    Box::new(MapNone {
                        val: HashMap::from([(123, true), (456, false)]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_min_pairs_valid",
            Box::new(|| {
                (
                    Box::new(MapMin {
                        val: HashMap::from([(1, 2.), (3, 4.), (5, 6.)]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_min_pairs_valid_equal",
            Box::new(|| {
                (
                    Box::new(MapMin {
                        val: HashMap::from([(1, 2.), (3, 4.)]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_min_pairs_invalid",
            Box::new(|| {
                (
                    Box::new(MapMin {
                        val: HashMap::from([(1, 2.)]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_max_pairs_valid",
            Box::new(|| {
                (
                    Box::new(MapMax {
                        val: HashMap::from([(1, 2.), (3, 4.)]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_max_pairs_valid_equal",
            Box::new(|| {
                (
                    Box::new(MapMax {
                        val: HashMap::from([(1, 2.), (3, 4.), (5, 6.)]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_max_pairs_invalid",
            Box::new(|| {
                (
                    Box::new(MapMax {
                        val: HashMap::from([(1, 2.), (3, 4.), (5, 6.), (7, 8.)]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_min_max_valid",
            Box::new(|| {
                (
                    Box::new(MapMinMax {
                        val: HashMap::from([
                            ("a".to_string(), true),
                            ("b".to_string(), false),
                            ("c".to_string(), true),
                        ]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_min_max_valid_min",
            Box::new(|| {
                (
                    Box::new(MapMinMax {
                        val: HashMap::from([("a".to_string(), true), ("b".to_string(), false)]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_min_max_valid_max",
            Box::new(|| {
                (
                    Box::new(MapMinMax {
                        val: HashMap::from([
                            ("a".to_string(), true),
                            ("b".to_string(), false),
                            ("c".to_string(), true),
                            ("d".to_string(), false),
                        ]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_min_max_invalid_below",
            Box::new(|| {
                (
                    Box::new(MapMinMax {
                        val: HashMap::from([]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_min_max_invalid_above",
            Box::new(|| {
                (
                    Box::new(MapMinMax {
                        val: HashMap::from([
                            ("a".to_string(), true),
                            ("b".to_string(), false),
                            ("c".to_string(), true),
                            ("d".to_string(), false),
                            ("e".to_string(), true),
                        ]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_exact_valid",
            Box::new(|| {
                (
                    Box::new(MapExact {
                        val: HashMap::from([
                            (1, "a".to_string()),
                            (2, "b".to_string()),
                            (3, "c".to_string()),
                        ]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_exact_invalid_below",
            Box::new(|| {
                (
                    Box::new(MapExact {
                        val: HashMap::from([(1, "a".to_string()), (2, "b".to_string())]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_exact_invalid_above",
            Box::new(|| {
                (
                    Box::new(MapExact {
                        val: HashMap::from([
                            (1, "a".to_string()),
                            (2, "b".to_string()),
                            (3, "c".to_string()),
                            (4, "d".to_string()),
                        ]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_no_sparse_valid",
            Box::new(|| {
                (
                    Box::new(MapNoSparse {
                        val: HashMap::from([
                            (1, cases::map_no_sparse::Msg::default()),
                            (2, cases::map_no_sparse::Msg::default()),
                        ]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_no_sparse_valid_empty",
            Box::new(|| {
                (
                    Box::new(MapNoSparse {
                        val: HashMap::from([]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        // sparse maps are no longer supported, so this case is no longer possible
        // "map_no_sparse_invalid", MapNoSparse{val: HashMap::from([(1, cases::map_no_sparse::Msg::default()), (2, None)])}, 1),
        (
            "map_keys_valid",
            Box::new(|| {
                (
                    Box::new(MapKeys {
                        val: HashMap::from([(-1, "a".to_string()), (-2, "b".to_string())]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_keys_valid_empty",
            Box::new(|| {
                (
                    Box::new(MapKeys {
                        val: HashMap::default(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_keys_valid_pattern",
            Box::new(|| {
                (
                    Box::new(MapKeysPattern {
                        val: HashMap::from([("A".to_string(), "a".to_string())]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_keys_valid_in",
            Box::new(|| {
                (
                    Box::new(MapKeysIn {
                        val: HashMap::from([("foo".to_string(), "value".to_string())]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_keys_valid_not_in",
            Box::new(|| {
                (
                    Box::new(MapKeysNotIn {
                        val: HashMap::from([("baz".to_string(), "value".to_string())]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_keys_invalid",
            Box::new(|| {
                (
                    Box::new(MapKeys {
                        val: HashMap::from([(1, "a".to_string())]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_keys_invalid_pattern",
            Box::new(|| {
                (
                    Box::new(MapKeysPattern {
                        val: HashMap::from([
                            ("A".to_string(), "a".to_string()),
                            ("!@#$%^&*()".to_string(), "b".to_string()),
                        ]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_keys_invalid_in",
            Box::new(|| {
                (
                    Box::new(MapKeysIn {
                        val: HashMap::from([("baz".to_string(), "value".to_string())]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_keys_invalid_not_in",
            Box::new(|| {
                (
                    Box::new(MapKeysNotIn {
                        val: HashMap::from([("foo".to_string(), "value".to_string())]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_values_valid",
            Box::new(|| {
                (
                    Box::new(MapValues {
                        val: HashMap::from([
                            ("a".to_string(), "Alpha".to_string()),
                            ("b".to_string(), "Beta".to_string()),
                        ]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_values_valid_empty",
            Box::new(|| {
                (
                    Box::new(MapValues {
                        val: HashMap::default(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_values_valid_pattern",
            Box::new(|| {
                (
                    Box::new(MapValuesPattern {
                        val: HashMap::from([("a".to_string(), "A".to_string())]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_values_invalid",
            Box::new(|| {
                (
                    Box::new(MapValues {
                        val: HashMap::from([
                            ("a".to_string(), "A".to_string()),
                            ("b".to_string(), "B".to_string()),
                        ]),
                    }) as Box<dyn Validator>,
                    2,
                )
            }) as Factory,
        ),
        (
            "map_values_invalid_pattern",
            Box::new(|| {
                (
                    Box::new(MapValuesPattern {
                        val: HashMap::from([
                            ("a".to_string(), "A".to_string()),
                            ("b".to_string(), "!@#$%^&*()".to_string()),
                        ]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_recursive_valid",
            Box::new(|| {
                (
                    Box::new(MapRecursive {
                        val: HashMap::from([(
                            1,
                            cases::map_recursive::Msg {
                                val: "abc".to_string(),
                            },
                        )]),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "map_recursive_invalid",
            Box::new(|| {
                (
                    Box::new(MapRecursive {
                        val: HashMap::from([(1, cases::map_recursive::Msg::default())]),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "map_exact_valid_ignore_empty",
            Box::new(|| (Box::new(MapExactIgnore::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "map_multiple_valid",
            Box::new(|| {
                (
                    Box::new(MultipleMaps {
                        first: HashMap::from([(1, "a".to_string()), (2, "b".to_string())]),
                        second: HashMap::from([(-1, true), (-2, false)]),
                        third: HashMap::default(),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_none_valid",
            Box::new(|| {
                (
                    Box::new(OneOfNone {
                        o: Some(one_of_none::O::X("foo".to_string())),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_none_valid_empty",
            Box::new(|| (Box::new(OneOfNone::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "oneof_field_valid_x",
            Box::new(|| {
                (
                    Box::new(OneOf {
                        o: Some(one_of::O::X("foobar".to_string())),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_field_valid_y",
            Box::new(|| {
                (
                    Box::new(OneOf {
                        o: Some(one_of::O::Y(123)),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_field_valid_z",
            Box::new(|| {
                (
                    Box::new(OneOf {
                        o: Some(one_of::O::Z(TestOneOfMsg { val: true })),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_field_valid_empty",
            Box::new(|| (Box::new(OneOf::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "oneof_field_invalid_x",
            Box::new(|| {
                (
                    Box::new(OneOf {
                        o: Some(one_of::O::X("fizzbuzz".to_string())),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "oneof_field_invalid_y",
            Box::new(|| {
                (
                    Box::new(OneOf {
                        o: Some(one_of::O::Y(-1)),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "oneof_filed_invalid_z",
            Box::new(|| {
                (
                    Box::new(OneOf {
                        o: Some(one_of::O::Z(TestOneOfMsg::default())),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "oneof_required_valid",
            Box::new(|| {
                (
                    Box::new(OneOfRequired {
                        o: Some(one_of_required::O::X("".to_string())),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_require_invalid",
            Box::new(|| (Box::new(OneOfRequired::default()) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "oneof_ignore_empty_valid_x",
            Box::new(|| {
                (
                    Box::new(OneOfIgnoreEmpty {
                        o: Some(one_of_ignore_empty::O::X("".to_string())),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_ignore_empty_valid_y",
            Box::new(|| {
                (
                    Box::new(OneOfIgnoreEmpty {
                        o: Some(one_of_ignore_empty::O::Y(b"".to_vec())),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "oneof_ignore_empty_valid_z",
            Box::new(|| {
                (
                    Box::new(OneOfIgnoreEmpty {
                        o: Some(one_of_ignore_empty::O::Z(0)),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_none_valid",
            Box::new(|| {
                (
                    Box::new(WrapperNone { val: Some(123) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_none_valid_empty",
            Box::new(|| (Box::new(WrapperNone { val: None }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "wrapper_float_valid",
            Box::new(|| {
                (
                    Box::new(WrapperFloat { val: Some(1.) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_float_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperFloat { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_float_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperFloat { val: Some(0.) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_double_valid",
            Box::new(|| {
                (
                    Box::new(WrapperDouble { val: Some(1.) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_double_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperDouble { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_double_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperDouble { val: Some(0.) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_int64_valid",
            Box::new(|| {
                (
                    Box::new(WrapperInt64 { val: Some(1) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_int64_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperInt64 { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_int64_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperInt64 { val: Some(0) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_int32_valid",
            Box::new(|| {
                (
                    Box::new(WrapperInt32 { val: Some(1) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_int32_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperInt32 { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_int32_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperInt32 { val: Some(0) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_uint64_valid",
            Box::new(|| {
                (
                    Box::new(WrapperUInt64 { val: Some(1) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_uint64_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperUInt64 { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_uint64_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperUInt64 { val: Some(0) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_uint32_valid",
            Box::new(|| {
                (
                    Box::new(WrapperUInt32 { val: Some(1) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_uint32_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperUInt32 { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_uint32_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperUInt32 { val: Some(0) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_bool_valid",
            Box::new(|| {
                (
                    Box::new(WrapperBool { val: Some(true) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_bool_valid_empty",
            Box::new(|| (Box::new(WrapperBool { val: None }) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "wrapper_bool_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperBool { val: Some(false) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_string_valid",
            Box::new(|| {
                (
                    Box::new(WrapperString {
                        val: Some("foobar".to_string()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_string_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperString { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_string_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperString {
                        val: Some("fizzbuzz".to_string()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_bytes_valid",
            Box::new(|| {
                (
                    Box::new(WrapperBytes {
                        val: Some(b"foo".to_vec()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_bytes_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperBytes { val: None }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_bytes_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperBytes {
                        val: Some(b"x".to_vec()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_string_valid",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredString {
                        val: Some("bar".to_string()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_string_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredString {
                        val: Some("foo".to_string()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_string_invalid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredString::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_string_empty_valid",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredEmptyString {
                        val: Some("".to_string()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_string_empty_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredEmptyString {
                        val: Some("foo".to_string()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_string_empty_invalid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredEmptyString::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_optional_string_uuid_valid",
            Box::new(|| {
                (
                    Box::new(WrapperOptionalUuidString {
                        val: Some("8b72987b-024a-43b3-b4cf-647a1f925c5d".to_string()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_optional_string_uuid_valid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperOptionalUuidString::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_optional_string_uuid_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperOptionalUuidString {
                        val: Some("foo".to_string()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_float_valid",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredFloat { val: Some(1.) }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_float_invalid",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredFloat { val: Some(-5.) }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "wrapper_required_float_invalid_empty",
            Box::new(|| {
                (
                    Box::new(WrapperRequiredFloat::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_none_valid",
            Box::new(|| {
                (
                    Box::new(DurationNone {
                        val: Some(Duration {
                            seconds: 123,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_required_valid",
            Box::new(|| {
                (
                    Box::new(DurationRequired {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_required_invalid",
            Box::new(|| {
                (
                    Box::new(DurationRequired::default()) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_const_valid",
            Box::new(|| {
                (
                    Box::new(DurationConst {
                        val: Some(Duration {
                            seconds: 3,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_const_valid_empty",
            Box::new(|| (Box::new(DurationConst::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_const_invalid",
            Box::new(|| {
                (
                    Box::new(DurationConst {
                        val: Some(Duration {
                            nanos: 3,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_in_valid",
            Box::new(|| {
                (
                    Box::new(DurationIn {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_in_valid_empty",
            Box::new(|| (Box::new(DurationIn::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_in_invalid",
            Box::new(|| {
                (
                    Box::new(DurationIn {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_not_in_valid",
            Box::new(|| {
                (
                    Box::new(DurationNotIn {
                        val: Some(Duration {
                            nanos: 1,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_not_in_valid_empty",
            Box::new(|| (Box::new(DurationNotIn::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(DurationNotIn {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_lt_valid",
            Box::new(|| {
                (
                    Box::new(DurationLt {
                        val: Some(Duration {
                            nanos: -1,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_lt_valid_empty",
            Box::new(|| (Box::new(DurationLt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_lt_invalid_equal",
            Box::new(|| {
                (
                    Box::new(DurationLt {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_lt_invalid",
            Box::new(|| {
                (
                    Box::new(DurationLt {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_lte_valid",
            Box::new(|| {
                (
                    Box::new(DurationLte {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_lte_valid_empty",
            Box::new(|| (Box::new(DurationLte::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_lte_valid_equal",
            Box::new(|| {
                (
                    Box::new(DurationLte {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_lte_invalid",
            Box::new(|| {
                (
                    Box::new(DurationLte {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 1,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gt_valid",
            Box::new(|| {
                (
                    Box::new(DurationGt {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gt_valid_empty",
            Box::new(|| (Box::new(DurationGt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_gt_invalid_equal",
            Box::new(|| {
                (
                    Box::new(DurationGt {
                        val: Some(Duration {
                            nanos: 1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gt_invalid",
            Box::new(|| {
                (
                    Box::new(DurationGt {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gte_valid",
            Box::new(|| {
                (
                    Box::new(DurationGte {
                        val: Some(Duration {
                            seconds: 3,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gte_valid_empty",
            Box::new(|| (Box::new(DurationGte::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_gte_valid_equal",
            Box::new(|| {
                (
                    Box::new(DurationGte {
                        val: Some(Duration {
                            nanos: 1000000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gte_invalid",
            Box::new(|| {
                (
                    Box::new(DurationGte {
                        val: Some(Duration {
                            seconds: -1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gt_lt_valid",
            Box::new(|| {
                (
                    Box::new(DurationGtlt {
                        val: Some(Duration {
                            nanos: 1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gt_lt_valid_empty",
            Box::new(|| (Box::new(DurationGtlt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_gt_lt_invalid_above",
            Box::new(|| {
                (
                    Box::new(DurationGtlt {
                        val: Some(Duration {
                            seconds: 1000,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gt_lt_invalid_below",
            Box::new(|| {
                (
                    Box::new(DurationGtlt {
                        val: Some(Duration {
                            nanos: -1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gt_lt_invalid_max",
            Box::new(|| {
                (
                    Box::new(DurationGtlt {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gt_lt_invalid_min",
            Box::new(|| {
                (
                    Box::new(DurationGtlt {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gt_lt_valid_empty",
            Box::new(|| (Box::new(DurationExLtgt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_exclusive_gt_lt_valid_above",
            Box::new(|| {
                (
                    Box::new(DurationExLtgt {
                        val: Some(Duration {
                            seconds: 2,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gt_lt_valid_below",
            Box::new(|| {
                (
                    Box::new(DurationExLtgt {
                        val: Some(Duration {
                            nanos: -1,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gt_lt_invalid",
            Box::new(|| {
                (
                    Box::new(DurationExLtgt {
                        val: Some(Duration {
                            nanos: 1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gt_lt_invalid_max",
            Box::new(|| {
                (
                    Box::new(DurationExLtgt {
                        val: Some(Duration {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gt_lt_invalid_min",
            Box::new(|| {
                (
                    Box::new(DurationExLtgt {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(DurationGtelte {
                        val: Some(Duration {
                            seconds: 60,
                            nanos: 1,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gte_lte_valid_empty",
            Box::new(|| (Box::new(DurationGtelte::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "duration_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(DurationGtelte {
                        val: Some(Duration {
                            seconds: 3600,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(DurationGtelte {
                        val: Some(Duration {
                            seconds: 60,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(DurationGtelte {
                        val: Some(Duration {
                            seconds: 3600,
                            nanos: 1,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(DurationGtelte {
                        val: Some(Duration {
                            seconds: 59,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_gte_lte_valid_empty_2",
            Box::new(|| {
                (
                    Box::new(DurationExGtelte::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(DurationExGtelte {
                        val: Some(Duration {
                            seconds: 3601,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(DurationExGtelte {
                        val: Some(Duration::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(DurationExGtelte {
                        val: Some(Duration {
                            seconds: 3600,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(DurationExGtelte {
                        val: Some(Duration {
                            seconds: 60,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "duration_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(DurationExGtelte {
                        val: Some(Duration {
                            seconds: 61,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "duration_fields_with_other_fields_invalid_other_field",
            Box::new(|| {
                (
                    Box::new(DurationFieldWithOtherFields {
                        duration_val: None,
                        int_val: 12,
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_none_valid",
            Box::new(|| {
                (
                    Box::new(TimestampNone {
                        val: Some(Timestamp {
                            seconds: 123,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_required_valid",
            Box::new(|| {
                (
                    Box::new(TimestampRequired {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_required_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampRequired { val: None }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_const_valid",
            Box::new(|| {
                (
                    Box::new(TimestampConst {
                        val: Some(Timestamp {
                            seconds: 3,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_const_valid_empty",
            Box::new(|| (Box::new(TimestampConst::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_const_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampConst {
                        val: Some(Timestamp {
                            nanos: 3,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_valid",
            Box::new(|| {
                (
                    Box::new(TimestampLt {
                        val: Some(Timestamp {
                            seconds: -1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_valid_empty",
            Box::new(|| (Box::new(TimestampLt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_lt_invalid_equal",
            Box::new(|| {
                (
                    Box::new(TimestampLt {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampLt {
                        val: Some(Timestamp {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_lte_valid",
            Box::new(|| {
                (
                    Box::new(TimestampLte {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_lte_valid_empty",
            Box::new(|| (Box::new(TimestampLte::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_lte_valid_equal",
            Box::new(|| {
                (
                    Box::new(TimestampLte {
                        val: Some(Timestamp {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_lte_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampLte {
                        val: Some(Timestamp {
                            seconds: 1,
                            nanos: 1,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_valid",
            Box::new(|| {
                (
                    Box::new(TimestampGt {
                        val: Some(Timestamp {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_valid_empty",
            Box::new(|| (Box::new(TimestampGt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_gt_invalid_equal",
            Box::new(|| {
                (
                    Box::new(TimestampGt {
                        val: Some(Timestamp {
                            nanos: 1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampGt {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_valid",
            Box::new(|| {
                (
                    Box::new(TimestampGte {
                        val: Some(Timestamp {
                            seconds: 3,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_valid_empty",
            Box::new(|| (Box::new(TimestampGte::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_gte_valid_equal",
            Box::new(|| {
                (
                    Box::new(TimestampGte {
                        val: Some(Timestamp {
                            nanos: 1000000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampGte {
                        val: Some(Timestamp {
                            seconds: -1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_lt_valid",
            Box::new(|| {
                (
                    Box::new(TimestampGtlt {
                        val: Some(Timestamp {
                            nanos: 1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_lt_valid_empty",
            Box::new(|| (Box::new(TimestampGtlt::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_gt_lt_invalid_above",
            Box::new(|| {
                (
                    Box::new(TimestampGtlt {
                        val: Some(Timestamp {
                            seconds: 1000,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_lt_invalid_below",
            Box::new(|| {
                (
                    Box::new(TimestampGtlt {
                        val: Some(Timestamp {
                            seconds: -1000,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_lt_invalid_max",
            Box::new(|| {
                (
                    Box::new(TimestampGtlt {
                        val: Some(Timestamp {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_lt_invalid_min",
            Box::new(|| {
                (
                    Box::new(TimestampGtlt {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gt_lt_valid_empty",
            Box::new(|| {
                (
                    Box::new(TimestampExLtgt::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gt_lt_valid_above",
            Box::new(|| {
                (
                    Box::new(TimestampExLtgt {
                        val: Some(Timestamp {
                            seconds: 2,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gt_lt_valid_below",
            Box::new(|| {
                (
                    Box::new(TimestampExLtgt {
                        val: Some(Timestamp {
                            seconds: -1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gt_lt_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampExLtgt {
                        val: Some(Timestamp {
                            nanos: 1000,
                            seconds: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gt_lt_invalid_max",
            Box::new(|| {
                (
                    Box::new(TimestampExLtgt {
                        val: Some(Timestamp {
                            seconds: 1,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gt_lt_invalid_min",
            Box::new(|| {
                (
                    Box::new(TimestampExLtgt {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_valid",
            Box::new(|| {
                (
                    Box::new(TimestampGtelte {
                        val: Some(Timestamp {
                            seconds: 60,
                            nanos: 1,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_valid_empty",
            Box::new(|| {
                (
                    Box::new(TimestampGtelte::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(TimestampGtelte {
                        val: Some(Timestamp {
                            seconds: 3600,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(TimestampGtelte {
                        val: Some(Timestamp {
                            seconds: 60,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_invalid_above",
            Box::new(|| {
                (
                    Box::new(TimestampGtelte {
                        val: Some(Timestamp {
                            seconds: 3600,
                            nanos: 1,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_invalid_below",
            Box::new(|| {
                (
                    Box::new(TimestampGtelte {
                        val: Some(Timestamp {
                            seconds: 59,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gte_lte_valid_empty_2",
            Box::new(|| {
                (
                    Box::new(TimestampExGtelte::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gte_lte_valid_above",
            Box::new(|| {
                (
                    Box::new(TimestampExGtelte {
                        val: Some(Timestamp {
                            seconds: 3601,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gte_lte_valid_below",
            Box::new(|| {
                (
                    Box::new(TimestampExGtelte {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gte_lte_valid_max",
            Box::new(|| {
                (
                    Box::new(TimestampExGtelte {
                        val: Some(Timestamp {
                            seconds: 3600,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gte_lte_valid_min",
            Box::new(|| {
                (
                    Box::new(TimestampExGtelte {
                        val: Some(Timestamp {
                            seconds: 60,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_exclusive_gte_lte_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampExGtelte {
                        val: Some(Timestamp {
                            seconds: 61,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_now_valid",
            Box::new(|| {
                (
                    Box::new(TimestampLtNow {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_now_valid_empty",
            Box::new(|| (Box::new(TimestampLtNow::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_lt_now_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampLtNow {
                        val: Some(Timestamp {
                            seconds: now() + 7200,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_now_valid",
            Box::new(|| {
                (
                    Box::new(TimestampGtNow {
                        val: Some(Timestamp {
                            seconds: now() + 7200,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_now_valid_empty",
            Box::new(|| (Box::new(TimestampGtNow::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "timestamp_gt_now_invalid",
            Box::new(|| {
                (
                    Box::new(TimestampGtNow {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_within_valid",
            Box::new(|| {
                (
                    Box::new(TimestampWithin {
                        val: Some(Timestamp {
                            seconds: now(),
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_within_valid_empty",
            Box::new(|| {
                (
                    Box::new(TimestampWithin::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_within_invalid_below",
            Box::new(|| {
                (
                    Box::new(TimestampWithin {
                        val: Some(Timestamp::default()),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_within_invalid_above",
            Box::new(|| {
                (
                    Box::new(TimestampWithin {
                        val: Some(Timestamp {
                            seconds: now() + 7200,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_now_within_valid",
            Box::new(|| {
                (
                    Box::new(TimestampLtNowWithin {
                        val: Some(Timestamp {
                            seconds: now() - 1800,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_now_within_valid_empty",
            Box::new(|| {
                (
                    Box::new(TimestampLtNowWithin::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_now_within_invalid_lt",
            Box::new(|| {
                (
                    Box::new(TimestampLtNowWithin {
                        val: Some(Timestamp {
                            seconds: now() + 1800,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_lt_now_within_invalid_within",
            Box::new(|| {
                (
                    Box::new(TimestampLtNowWithin {
                        val: Some(Timestamp {
                            seconds: now() - 7200,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_now_within_valid",
            Box::new(|| {
                (
                    Box::new(TimestampGtNowWithin {
                        val: Some(Timestamp {
                            seconds: now() + 1800,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_now_within_valid_empty",
            Box::new(|| {
                (
                    Box::new(TimestampGtNowWithin::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_now_within_invalid_gt",
            Box::new(|| {
                (
                    Box::new(TimestampGtNowWithin {
                        val: Some(Timestamp {
                            seconds: now() - 1800,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "timestamp_gt_now_within_invalid_within",
            Box::new(|| {
                (
                    Box::new(TimestampGtNowWithin {
                        val: Some(Timestamp {
                            seconds: now() + 7200,
                            nanos: 0,
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "any_none_valid",
            Box::new(|| {
                (
                    Box::new(AnyNone {
                        val: Some(Any::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "any_required_valid",
            Box::new(|| {
                (
                    Box::new(AnyRequired {
                        val: Some(Any::default()),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "any_required_invalid",
            Box::new(|| (Box::new(AnyRequired { val: None }) as Box<dyn Validator>, 1)) as Factory,
        ),
        (
            "any_in_valid",
            Box::new(|| {
                (
                    Box::new(AnyIn {
                        val: Some(Any {
                            type_url: "type.googleapis.com/google.protobuf.Duration".to_string(),
                            ..Any::default()
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "any_in_valid_empty",
            Box::new(|| (Box::new(AnyIn::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "any_in_invalid",
            Box::new(|| {
                (
                    Box::new(AnyIn {
                        val: Some(Any {
                            type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(),
                            ..Any::default()
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "any_not_in_valid",
            Box::new(|| {
                (
                    Box::new(AnyNotIn {
                        val: Some(Any {
                            type_url: "type.googleapis.com/google.protobuf.Duration".to_string(),
                            ..Any::default()
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "any_not_in_valid_empty",
            Box::new(|| (Box::new(AnyNotIn::default()) as Box<dyn Validator>, 0)) as Factory,
        ),
        (
            "any_not_in_invalid",
            Box::new(|| {
                (
                    Box::new(AnyNotIn {
                        val: Some(Any {
                            type_url: "type.googleapis.com/google.protobuf.Timestamp".to_string(),
                            ..Any::default()
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
        (
            "kitchensink_field_valid",
            Box::new(|| {
                (
                    Box::new(KitchenSinkMessage {
                        val: Some(ComplexTestMsg {
                            r#const: "abcd".to_string(),
                            int_const: 5,
                            bool_const: false,
                            float_val: Some(1.),
                            dur_val: Some(Duration {
                                seconds: 3,
                                nanos: 0,
                            }),
                            ts_val: Some(Timestamp {
                                seconds: 17,
                                nanos: 0,
                            }),
                            float_const: 7.,
                            double_in: 123.,
                            enum_const: ComplexTestEnum::ComplexTwo.into(),
                            any_val: Some(Any {
                                type_url: "type.googleapis.com/google.protobuf.Duration"
                                    .to_string(),
                                ..Any::default()
                            }),
                            rep_ts_val: vec![Timestamp {
                                seconds: 3,
                                nanos: 0,
                            }],
                            map_val: HashMap::from([(-1, "a".to_string()), (-2, "b".to_string())]),
                            bytes_val: b"\x00\x99".to_vec(),
                            o: Some(complex_test_msg::O::X("foobar".to_string())),
                            ..ComplexTestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "kitchensink_valid_unset",
            Box::new(|| {
                (
                    Box::new(KitchenSinkMessage::default()) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "kitchensink_field_invalid",
            Box::new(|| {
                (
                    Box::new(KitchenSinkMessage {
                        val: Some(ComplexTestMsg::default()),
                    }) as Box<dyn Validator>,
                    7,
                )
            }) as Factory,
        ),
        (
            "kitchensink_field_embedded_invalid",
            Box::new(|| {
                (
                    Box::new(KitchenSinkMessage {
                        val: Some(ComplexTestMsg {
                            another: Some(Box::new(ComplexTestMsg::default())),
                            ..ComplexTestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    14,
                )
            }) as Factory,
        ),
        (
            "kitchensink_field_invalid_transitive",
            Box::new(|| {
                (
                    Box::new(KitchenSinkMessage {
                        val: Some(ComplexTestMsg {
                            r#const: "abcd".to_string(),
                            bool_const: true,
                            nested: Some(Box::new(ComplexTestMsg::default())),
                            ..ComplexTestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    14,
                )
            }) as Factory,
        ),
        (
            "kitchensink_many_all_non_message_fields_invalid",
            Box::new(|| {
                (
                    Box::new(KitchenSinkMessage {
                        val: Some(ComplexTestMsg {
                            bool_const: true,
                            float_val: Some(0.),
                            ts_val: Some(Timestamp::default()),
                            float_const: 8.,
                            any_val: Some(Any {
                                type_url: "asdf".to_string(),
                                ..Any::default()
                            }),
                            rep_ts_val: vec![Timestamp {
                                seconds: 0,
                                nanos: 1,
                            }],
                            ..ComplexTestMsg::default()
                        }),
                    }) as Box<dyn Validator>,
                    13,
                )
            }) as Factory,
        ),
        (
            "nested_wkt_uuid_field_valid",
            Box::new(|| {
                (
                    Box::new(WktLevelOne {
                        two: Some(cases::wkt_level_one::WktLevelTwo {
                            three: Some(cases::wkt_level_one::wkt_level_two::WktLevelThree {
                                uuid: "f81d16ef-40e2-40c6-bebc-89aaf5292f9a".to_string(),
                            }),
                        }),
                    }) as Box<dyn Validator>,
                    0,
                )
            }) as Factory,
        ),
        (
            "nested_wkt_uuid_field_invalid",
            Box::new(|| {
                (
                    Box::new(WktLevelOne {
                        two: Some(cases::wkt_level_one::WktLevelTwo {
                            three: Some(cases::wkt_level_one::wkt_level_two::WktLevelThree {
                                uuid: "not-a-valid-uuid".to_string(),
                            }),
                        }),
                    }) as Box<dyn Validator>,
                    1,
                )
            }) as Factory,
        ),
    ])
});
