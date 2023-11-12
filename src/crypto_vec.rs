use std::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct CryptoVec<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> Drop for CryptoVec<N> {
    fn drop(&mut self) {
        for i in 0..N {
            self.inner[i] = 0;
        }
    }
}

impl<const N: usize> From<[u8; N]> for CryptoVec<N> {
    fn from(value: [u8; N]) -> Self {
        CryptoVec { inner: value }
    }
}

impl<const N: usize> AsRef<[u8]> for CryptoVec<N> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<const N: usize> AsMut<[u8]> for CryptoVec<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl<const N: usize> Index<usize> for CryptoVec<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const N: usize> IndexMut<usize> for CryptoVec<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
