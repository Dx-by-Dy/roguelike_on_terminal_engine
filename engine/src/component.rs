use std::any::Any;

use engine_macros::Component;

pub mod point;
pub mod pointer;
pub mod positions;
pub mod size;
pub mod timestamp;
pub mod transformation;
pub mod user_event;

// #[derive(Debug, Clone)]
// pub struct ComponentVec {
//     pub value: Vec<Component>,
// }

pub trait Component: Any {
    // fn type_id(&self) -> usize;

    // fn static_type_id() -> usize
    // where
    //     Self: Sized;
}

// impl<A: Component, B: Component> TryInto<A> for B {
//     type Error = ();

//     fn try_into(self) -> Result<A, Self::Error> {
//         if  == self.type_id() {
//             return Some(from.into());
//         }
//         None
//     }
// }

// #[macro_export]
// macro_rules! define_components {
//     ( $( $type:ident ),* $(,)? ) => {
//         #[derive(Debug, Clone)]
//         pub enum Component {
//             $(
//                 $type($type),
//             )*
//         }

//         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//         pub enum ComponentType {
//             $(
//                 $type,
//             )*
//         }

//         $(
//             impl AssociatedComponentType<Component, ComponentType> for $type {
//                 const ASSOCIATED_COMPONENT_TYPE: ComponentType =
//                     ComponentType::$type;
//             }

//             impl<'a> TryInto<&'a mut $type> for &'a mut Component {
//                 type Error = ();

//                 fn try_into(self) -> Result<&'a mut $type, Self::Error> {
//                     match self {
//                         Component::$type(value) => Ok(value),
//                         _ => Err(()),
//                     }
//                 }
//             }

//             impl<'a> TryInto<&'a $type> for &'a Component {
//                 type Error = ();

//                 fn try_into(self) -> Result<&'a $type, Self::Error> {
//                     match self {
//                         Component::$type(value) => Ok(value),
//                         _ => Err(()),
//                     }
//                 }
//             }

//             impl From<$type> for Component {
//                 fn from(value: $type) -> Self {
//                     Component::$type(value)
//                 }
//             }
//         )*
//     };
// }

// #[macro_export]
// macro_rules! define_names {
//     ( $( $type:ident ),* $(,)? ) => {
//         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//         pub enum PointerName {
//             $(
//                 $type,
//             )*
//         }
//     };
// }

// define_components! {
//     Pointer,
//     SurfacePosition,
//     TerminalPosition,
//     Size,
//     Point,
//     Timestamp,
//     UserEvent,
//     // Transformation,
//     ComponentVec
// }

// define_names! {
//     Player
// }
