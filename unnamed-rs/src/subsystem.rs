use std::{future::Future, sync::Arc};

use tokio::sync::{OwnedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

/*pub trait Subsystem: Sync {
    type PeriodicTimer: Future<Output = ()> + Send + 'static;
    fn periodic<'a>(&'a mut self) -> impl Future<Output=Self::PeriodicTimer> + Send;//impl Future<Output=impl Future<Output = ()> + Send + 'static> + Send + 'a;
}*/
/*pub trait Subsystem: Sync {
    type PeriodicTimer: Future<Output = ()> + Send + 'static;
    fn periodic<'a>(&'a mut self) -> impl Future<Output=Self::PeriodicTimer> + Send;//impl Future<Output=impl Future<Output = ()> + Send + 'static> + Send + 'a;
}*/

#[derive(Clone)]
pub struct View<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> View<T> {
    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().await
    }

    pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.blocking_read()
    }

    pub async fn read_owned(&self) -> OwnedRwLockReadGuard<T> {
        self.inner.clone().read_owned().await
    }

    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError> {
        self.inner.try_read()
    }

    pub fn try_read_owned(&self) -> Result<OwnedRwLockReadGuard<T>, TryLockError> {
        self.inner.clone().try_read_owned()
    }
}

pub struct Handle<T> {
    inner: Arc<RwLock<T>>,
}

trait AsyncFnHelper<I>: FnMut(I) -> Self::OutputFut {
    type FinalOut;
    type OutputFut: Future<Output = Self::FinalOut>;
}
impl<F, I, OF> AsyncFnHelper<I> for F
where
    F: FnMut(I) -> OF,
    OF: Future,
{
    type FinalOut = OF::Output;
    type OutputFut = OF;
}

impl<T> Handle<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }

    pub fn new_and_inner(value: T) -> (Self, Arc<RwLock<T>>) {
        let inner = Arc::new(RwLock::new(value));
        (
            Self {
                inner: inner.clone(),
            },
            inner,
        )
    }

    /*pub fn new_with_periodic<F, Fut, Timer>(value: T, periodic: F) -> Self
        where
            T: Send + Sync + 'static,
            Timer: Future<Output=()> + Send + 'static,
            for<'a> Fut: Future<Output = Timer> + Send + 'a,
            F: for<'a> Fn(&'a mut T) -> Fut + Send + 'static
    {
        let inner = Arc::new(RwLock::new(value));
        let arc_dg = Arc::downgrade(&inner);
        let _ = spawn(async move {
            while let Some(arc) = arc_dg.upgrade() {
                let mut subsystem_ref = arc.write().await;
                let future = periodic(&mut *subsystem_ref);
                drop(subsystem_ref);
                let wait = future.await;
                // wait after lock is dropped to reduce time held
                wait.await;
            }
        });
        Self { inner }
    }*/

    pub fn make_view(&self) -> View<T> {
        View {
            inner: self.inner.clone(),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.read().await
    }

    pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
        self.inner.blocking_read()
    }

    pub async fn read_owned(&self) -> OwnedRwLockReadGuard<T> {
        self.inner.clone().read_owned().await
    }

    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError> {
        self.inner.try_read()
    }

    pub fn try_read_owned(&self) -> Result<OwnedRwLockReadGuard<T>, TryLockError> {
        self.inner.clone().try_read_owned()
    }

    pub async fn write(&mut self) -> RwLockWriteGuard<'_, T> {
        self.inner.write().await
    }

    pub fn blocking_write(&mut self) -> RwLockWriteGuard<'_, T> {
        self.inner.blocking_write()
    }

    pub fn try_write(&mut self) -> Result<RwLockWriteGuard<'_, T>, TryLockError> {
        self.inner.try_write()
    }
}

struct Handle2<T> {
    inner: RwLock<T>,
}

impl<T> Handle2<T> {
    pub async fn read(&self) -> &T {
        todo!()
    }
}

/*impl<T> Handle<T> where T: Subsystem + Send + 'static {
    pub fn new_subsystem(value: T) -> Self {
        let inner = Arc::new(RwLock::new(value));
        let arc_dg = Arc::downgrade(&inner);
        let _ = spawn(async move {
            while let Some(arc) = arc_dg.upgrade() {
                let mut subsystem_ref = arc.write().await;
                let wait = subsystem_ref.periodic().await;
                drop(subsystem_ref);
                // wait after lock is dropped to reduce time held
                wait.await;
            }
        });
        Self { inner }
    }
}*/

impl<T> From<Arc<RwLock<T>>> for Handle<T> {
    fn from(value: Arc<RwLock<T>>) -> Self {
        Self { inner: value }
    }
}

#[macro_export]
macro_rules! subsystem_periodic {
    ($arc_subsystem:ident, $func_name:ident) => {{
        let arc_downgrade = ::std::sync::Arc::downgrade(&$arc_subsystem);
        // TODO: depends on tokio being present in user code?
        let _ = ::tokio::spawn(async move {
            while let Some(arc) = ::std::sync::Weak::upgrade(&arc_downgrade) {
                let mut subsystem_ref = ::tokio::sync::RwLock::write(&arc).await;
                let wait = subsystem_ref.$func_name().await;
                ::core::mem::drop(subsystem_ref);
                ::core::mem::drop(arc);
                // wait after lock and arc are dropped to reduce time held
                wait.await;
                // ensure that we aren't in a blocking loop
                ::tokio::task::yield_now().await;
            }
        });
    }};
}
