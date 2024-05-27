use core::{marker::PhantomData, num::NonZeroUsize};

use alloc::vec::Vec;

pub(crate) type EdgeIndex = usize;

pub(crate) type NodeIndex = usize;

pub(crate) struct Node<T> {
    pub(crate) id: NodeIndex,
    pub(crate) data: T,
}

impl<T> Node<T> {
    pub(crate) fn data(&self) -> &T {
        &self.data
    }
}

pub(crate) struct Edge<E> {
    pub(crate) id: EdgeIndex,
    pub(crate) source: NodeIndex,
    pub(crate) target: NodeIndex,
    pub(crate) data: E,
}

impl<E> Edge<E> {
    pub(crate) fn data(&self) -> &E {
        &self.data
    }
}
