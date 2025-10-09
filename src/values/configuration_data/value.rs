use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

use allocative::Allocative;
use serde::Serialize;
use starlark::any::ProvidesStaticType;
use starlark::collections::SmallMap;
use starlark::environment::Methods;
use starlark::environment::MethodsStatic;
use starlark::typing::Ty;
use starlark::values::type_repr::StarlarkTypeRepr;
use starlark::values::AllocFrozenValue;
use starlark::values::AllocValue;
use starlark::values::Coerce;
use starlark::values::Freeze;
use starlark::values::FreezeError;
use starlark::values::FreezeResult;
use starlark::values::Freezer;
use starlark::values::FrozenHeap;
use starlark::values::FrozenValue;
use starlark::values::Heap;
use starlark::values::StarlarkValue;
use starlark::values::Value;
use starlark_derive::starlark_value;
use starlark_derive::Trace;

#[derive(Clone, Default, Trace, Debug, ProvidesStaticType, Allocative)]
pub(crate) struct CDGen<T>(pub(crate) T);

impl<'v, T: CDLike<'v>> Display for CDGen<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration_data")
    }
}

impl<'v, T: CDLike<'v>> Serialize for CDGen<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str("configuration_data")
    }
}

pub(crate) fn configuration_data_methods() -> Option<&'static Methods> {
    static RES: MethodsStatic = MethodsStatic::new();
    RES.methods(super::methods::configuration_data_methods)
}

#[starlark_value(type = "configuration_data")]
impl<'v, T: CDLike<'v> + 'v> StarlarkValue<'v> for CDGen<T>
where
    Self: ProvidesStaticType<'v>,
{
    type Canonical = FrozenCD;

    fn get_methods() -> Option<&'static Methods> {
        configuration_data_methods()
    }
}

pub(crate) type FrozenCD = CDGen<FrozenCDData>;
// pub(crate) type MutableCD<'v> = CDGen<RefCell<CD<'v>>>;

// Can convert from unfrozen to frozen.
unsafe impl<'v> Coerce<CD<'v>> for FrozenCDData {}

#[derive(Clone, Default, Trace, Debug, ProvidesStaticType, Allocative)]
pub(crate) struct ValueAndDescription<'v> {
    pub description: Option<Value<'v>>,
    pub value: Value<'v>,
}

/// Unfrozen CD
#[derive(Clone, Default, Trace, Debug, ProvidesStaticType, Allocative)]
#[repr(transparent)]
pub struct CD<'v> {
    pub(crate) content: SmallMap<String, ValueAndDescription<'v>>,
}

impl<'v> Display for CD<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration_data")
    }
}

impl<'v> StarlarkTypeRepr for CD<'v> {
    type Canonical = Self;
    fn starlark_type_repr() -> Ty {
        Ty::any()
    }
}

impl<'v> AllocValue<'v> for CD<'v> {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_complex(CDGen(RefCell::new(self)))
    }
}

trait CDLike<'v>: Debug + Allocative {}

impl<'v> CDLike<'v> for RefCell<CD<'v>> {}

// Frozen CDData
#[derive(Clone, Default, Debug, ProvidesStaticType, Allocative)]
pub(crate) struct FrozenCDData {
    pub(crate) content: SmallMap<FrozenValue, FrozenValue>,
}

impl<'v> CDLike<'v> for FrozenCDData {}

impl StarlarkTypeRepr for FrozenCDData {
    type Canonical = Self;

    fn starlark_type_repr() -> Ty {
        Ty::dict(Ty::any(), Ty::any())
    }
}

impl AllocFrozenValue for FrozenCDData {
    fn alloc_frozen_value(self, heap: &FrozenHeap) -> FrozenValue {
        heap.alloc_simple(CDGen(self))
    }
}

// Freeze implementation for CD
impl<'v> Freeze for CDGen<RefCell<CD<'v>>> {
    type Frozen = CDGen<FrozenCDData>;
    fn freeze(self, _freezer: &Freezer) -> FreezeResult<Self::Frozen> {
        // let content = self.0.into_inner().content.freeze(freezer)?;
        // Ok(CDGen(FrozenCDData { content }))
        Err(FreezeError::new(
            "configuration_data can not passed between threads safely.".to_string(),
        ))
    }
}
