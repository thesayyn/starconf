use std::{cell::{Ref, RefCell, RefMut}, convert::Infallible, ops::Deref};

use dupe::Dupe;
use either::Either;
use starlark::{coerce::coerce, typing::Ty, values::{type_repr::StarlarkTypeRepr, FrozenValue, UnpackValue, Value, ValueError, ValueLike}};

use super::value::{CDGen, FrozenCDData, CD};

/// Borrowed `CD`.
pub struct CDRef<'v> {
    pub(crate) aref: Either<Ref<'v, CD<'v>>, &'v CD<'v>>,
}

impl<'v> Clone for CDRef<'v> {
    fn clone(&self) -> Self {
        match &self.aref {
            Either::Left(x) => CDRef {
                aref: Either::Left(Ref::clone(x)),
            },
            Either::Right(x) => CDRef {
                aref: Either::Right(*x),
            },
        }
    }
}

impl<'v> Dupe for CDRef<'v> {}


/// Mutably borrowed `Dict`.
pub struct CDMut<'v> {
    pub(crate) aref: RefMut<'v, CD<'v>>,
}

/// Reference to frozen `Dict`.
pub struct FrozenCDRef {
    dict: &'static FrozenCDData,
}

impl<'v> CDRef<'v> {
    /// Downcast the value to a dict.
    pub fn from_value(x: Value<'v>) -> Option<CDRef<'v>> {
        if x.unpack_frozen().is_some() {
            x.downcast_ref::<CDGen<FrozenCDData>>()
                .map(|x| CDRef {
                    aref: Either::Right(coerce(&x.0)),
                })
        } else {
            let ptr = x.downcast_ref::<CDGen<RefCell<CD<'v>>>>()?;
            Some(CDRef {
                aref: Either::Left(ptr.0.borrow()),
            })
        }
    }
}

impl<'v> CDMut<'v> {
    /// Downcast the value to a mutable dict reference.
    #[inline]
    pub fn from_value(x: Value<'v>) -> anyhow::Result<CDMut<'v>> {
        #[derive(thiserror::Error, Debug)]
        #[error("Value is not dict, value type: `{0}`")]
        struct NotDictError(&'static str);

        #[cold]
        #[inline(never)]
        fn error<'v>(x: Value<'v>) -> anyhow::Error {
            if x.downcast_ref::<CDGen<FrozenCDData>>().is_some() {
                ValueError::CannotMutateImmutableValue.into()
            } else {
                NotDictError(x.get_type()).into()
            }
        }

        let ptr = x.downcast_ref::<CDGen<RefCell<CD<'v>>>>();
        match ptr {
            None => Err(error(x)),
            Some(ptr) => match ptr.0.try_borrow_mut() {
                Ok(x) => Ok(CDMut { aref: x }),
                Err(_) => Err(ValueError::MutationDuringIteration.into()),
            },
        }
    }
}

impl FrozenCDRef {
    /// Downcast to frozen dict.
    pub fn from_frozen_value(x: FrozenValue) -> Option<FrozenCDRef> {
        x.downcast_ref::<CDGen<FrozenCDData>>()
            .map(|x| FrozenCDRef { dict: &x.0 })
    }
}

impl<'v> Deref for CDRef<'v> {
    type Target = CD<'v>;

    fn deref(&self) -> &Self::Target {
        &self.aref
    }
}


impl<'v> StarlarkTypeRepr for CDRef<'v> {
    type Canonical = Self;

    fn starlark_type_repr() -> Ty {
      Ty::any()
    }
}


impl<'v> UnpackValue<'v> for CDRef<'v> {
    type Error = Infallible;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<CDRef<'v>>, Infallible> {
        Ok(CDRef::from_value(value))
    }
}