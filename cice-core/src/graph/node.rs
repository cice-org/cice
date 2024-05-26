use core::{marker::PhantomData, num::NonZeroUsize};

use alloc::vec::Vec;

pub(crate) type EdgeIndex = usize;

pub(crate) type NodeIndex = usize;

pub(crate) struct Node<T> {
    pub(crate) id: NodeIndex,
    pub(crate) data: T,
}

pub(crate) struct Edge<E> {
    pub(crate) id: EdgeIndex,
    pub(crate) source: NodeIndex,
    pub(crate) target: NodeIndex,
    pub(crate) data: E,
}
