#[macro_export]
macro_rules! try_observers {
    ($self:ident, $method:ident ( $($arg:expr),* )) => {
        {
            use std::rc::Rc;
            use std::cell::RefCell;
            use crate::Observer;

            $self.observers
                .iter()
                .try_for_each(|obs: &Rc<RefCell<dyn Observer>>| obs.borrow_mut().$method($($arg),*))
        }
    };
}

/// Macro for getting property from [`crate::Entity`].
///
/// # Examples
/// ```no_compile
/// let x: i32 = property!(entity, "property_name");
/// let y = property!(entity, i32, "property_name");
/// ```
#[macro_export]
macro_rules! property {
    ($ent:expr, $ty:ty, $fmt:literal, $($arg:tt)*) => {
        {
            let x: $ty = $ent.get_property_by_name(&format!($fmt, $($arg)*))?.try_into()?;
            x
        }
    };
    ($ent:expr, $ty:ty, $fmt:literal) => {
        {
            let x: $ty = $ent.get_property_by_name(&format!($fmt))?.try_into()?;
            x
        }
    };
    ($ent:expr, $fmt:expr, $($arg:tt)*) => {
        $ent.get_property_by_name(&format!($fmt, $($arg)*))?.try_into()?
    };
    ($ent:expr, $fmt:expr) => {{
        $ent.get_property_by_name(&format!($fmt))?.try_into()?
    }};
}

/// Same as [`crate::property`] but returns `None` if property doesn't exist for given
/// [`crate::Entity`] or cannot be converted into given type.
///
/// # Examples
/// ```no_compile
/// let x: i32 = try_property!(entity, "property_name").unwrap_or_default();
/// let y = try_property!(entity, i32, "property_name").unwrap_or_default();
/// ```
#[macro_export]
macro_rules! try_property {
    ($ent:expr, $ty:ty, $fmt:expr, $($arg:tt)*) => {
        {
            let x: Option<$ty> = $ent
                .get_property_by_name(&format!($fmt, $($arg)*))
                .ok()
                .and_then(|x| {
                    x.try_into().ok()
                });
            x
        }
    };

    ($ent:expr, $ty:ty, $fmt:expr) => {
        {
            let x: Option<$ty> = $ent
                .get_property_by_name(&format!($fmt))
                .ok()
                .and_then(|x| {
                    x.try_into().ok()
                });
            x
        }
    };

    ($ent:expr, $fmt:expr, $($arg:tt)*) => {
        $ent
            .get_property_by_name(&format!($fmt, $($arg)*))
            .ok()
            .and_then(|x| {
                x.try_into().ok()
            })
    };

    ($ent:expr, $fmt:expr) => {{
        $ent
            .get_property_by_name(&format!($fmt))
            .ok()
            .and_then(|x| {
                x.try_into().ok()
            })
    }};
}
