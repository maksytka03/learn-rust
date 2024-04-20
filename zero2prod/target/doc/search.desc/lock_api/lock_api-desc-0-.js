searchState.loadedDescShard("lock_api", 0, "This library provides type-safe and fully-featured <code>Mutex</code> …\nDuration type used for <code>try_lock_for</code>.\nDuration type used for <code>try_lock_for</code>.\nHelper trait which returns a non-zero thread ID.\nMarker type which determines whether a lock guard should …\nMarker type which determines whether a lock guard should …\nMarker type which indicates that the Guard type for a lock …\nMarker type which indicates that the Guard type for a lock …\nInitial value for an unlocked mutex.\nInitial value.\nInitial value for an unlocked mutex.\nInitial value for an unlocked <code>RwLock</code>.\nInstant type used for <code>try_lock_until</code>.\nInstant type used for <code>try_lock_until</code>.\nAn RAII mutex guard returned by <code>MutexGuard::map</code>, which can …\nAn RAII mutex guard returned by <code>ReentrantMutexGuard::map</code>, …\nAn RAII read lock guard returned by <code>RwLockReadGuard::map</code>, …\nAn RAII write lock guard returned by <code>RwLockWriteGuard::map</code>…\nA mutual exclusion primitive useful for protecting shared …\nAn RAII implementation of a “scoped lock” of a mutex. …\nBasic operations for a mutex.\nAdditional methods for mutexes which support fair …\nAdditional methods for mutexes which support locking with …\nA raw mutex type that wraps another raw mutex to provide …\nBasic operations for a reader-writer lock.\nAdditional methods for RwLocks which support atomically …\nAdditional methods for RwLocks which support fair …\nAdditional methods for RwLocks which support recursive …\nAdditional methods for RwLocks which support recursive …\nAdditional methods for RwLocks which support locking with …\nAdditional methods for RwLocks which support atomically …\nAdditional methods for RwLocks which support upgradable …\nAdditional methods for RwLocks which support upgradable …\nAdditional methods for RwLocks which support upgradable …\nA mutex which can be recursively locked by a single thread.\nAn RAII implementation of a “scoped lock” of a …\nA reader-writer lock\nRAII structure used to release the shared read access of a …\nRAII structure used to release the upgradable read access …\nRAII structure used to release the exclusive write access …\nTemporarily yields the mutex to a waiting thread if there …\nTemporarily yields the mutex to a waiting thread if there …\nTemporarily yields the mutex to a waiting thread if there …\nTemporarily yields the mutex to a waiting thread if there …\nTemporarily yields the mutex to a waiting thread if there …\nTemporarily yields the <code>RwLock</code> to a waiting thread if there …\nTemporarily yields the <code>RwLock</code> to a waiting thread if there …\nTemporarily yields the <code>RwLock</code> to a waiting thread if there …\nTemporarily yields an exclusive lock to a waiting thread …\nTemporarily yields an exclusive lock to a waiting thread …\nTemporarily yields a shared lock to a waiting thread if …\nTemporarily yields a shared lock to a waiting thread if …\nTemporarily yields an upgradable lock to a waiting thread …\nTemporarily yields an upgradable lock to a waiting thread …\nCreates a new mutex based on a pre-existing raw mutex.\nCreates a new reentrant mutex based on a pre-existing raw …\nCreates a new new instance of an <code>RwLock&lt;T&gt;</code> based on a …\nReturns a raw pointer to the underlying data.\nReturns a raw pointer to the underlying data.\nReturns a raw pointer to the underlying data.\nAtomically downgrades an exclusive lock into a shared lock …\nAtomically downgrades a write lock into a read lock …\nAtomically downgrades an upgradable read lock lock into a …\nDowngrades an exclusive lock to an upgradable lock.\nAtomically downgrades a write lock into an upgradable read …\nDowngrades an upgradable lock to a shared lock.\nForcibly unlocks the mutex.\nForcibly unlocks the mutex.\nForcibly unlocks the mutex using a fair unlock procotol.\nForcibly unlocks the mutex using a fair unlock protocol.\nForcibly unlocks a read lock.\nForcibly unlocks a read lock using a fair unlock procotol.\nForcibly unlocks a write lock.\nForcibly unlocks a write lock using a fair unlock procotol.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns a mutable reference to the underlying data.\nReturns a mutable reference to the underlying data.\nReturns a mutable reference to the underlying data.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes this mutex, returning the underlying data.\nConsumes this mutex, returning the underlying data.\nConsumes this <code>RwLock</code>, returning the underlying data.\nChecks whether the mutex is currently locked.\nChecks whether the mutex is currently locked.\nChecks whether the mutex is currently locked.\nChecks whether the mutex is currently locked.\nChecks whether the mutex is currently locked.\nChecks if this <code>RwLock</code> is currently locked in any way.\nChecks if this <code>RwLock</code> is currently locked in any way.\nChecks whether this <code>RwLock</code> is currently locked in any way.\nCheck if this <code>RwLock</code> is currently exclusively locked.\nCheck if this <code>RwLock</code> is currently exclusively locked.\nCheck if this <code>RwLock</code> is currently exclusively locked.\nChecks whether the mutex is currently held by the current …\nChecks whether the mutex is currently held by the current …\nLeaks the mutex guard and returns a mutable reference to …\nAcquires this mutex, blocking the current thread until it …\nAcquires a mutex, blocking the current thread until it is …\nAcquires this mutex, blocking if it’s held by another …\nAcquires a reentrant mutex, blocking the current thread …\nAcquires an exclusive lock, blocking the current thread …\nAcquires a shared lock, blocking the current thread until …\nAcquires a shared lock without deadlocking in case of a …\nAcquires an upgradable lock, blocking the current thread …\nCreates a new <code>MutexGuard</code> without checking if the mutex is …\nCreates a new <code>ReentrantMutexGuard</code> without checking if the …\nCreates a new <code>RwLockReadGuard</code> without checking if the lock …\nCreates a new <code>RwLockUpgradableReadGuard</code> without checking …\nCreates a new <code>RwLockReadGuard</code> without checking if the lock …\nMakes a new <code>MappedMutexGuard</code> for a component of the locked …\nMakes a new <code>MappedMutexGuard</code> for a component of the locked …\nMakes a new <code>MappedReentrantMutexGuard</code> for a component of …\nMakes a new <code>MappedReentrantMutexGuard</code> for a component of …\nMake a new <code>MappedRwLockReadGuard</code> for a component of the …\nMake a new <code>MappedRwLockWriteGuard</code> for a component of the …\nMake a new <code>MappedRwLockReadGuard</code> for a component of the …\nMake a new <code>MappedRwLockWriteGuard</code> for a component of the …\nReturns a reference to the original <code>Mutex</code> object.\nCreates a new mutex in an unlocked state ready for use.\nCreates a new reentrant mutex in an unlocked state ready …\nCreates a new instance of an <code>RwLock&lt;T&gt;</code> which is unlocked.\nReturns a non-zero thread ID which identifies the current …\nReturns the underlying raw mutex object.\nReturns the underlying raw mutex object.\nReturns the underlying raw reader-writer lock object.\nLocks this <code>RwLock</code> with shared read access, blocking the …\nLocks this <code>RwLock</code> with shared read access, blocking the …\nReturns a reference to the original <code>ReentrantMutex</code> object.\nReturns a reference to the original reader-writer lock …\nReturns a reference to the original reader-writer lock …\nReturns a reference to the original reader-writer lock …\nAttempts to acquire this mutex without blocking. Returns …\nAttempts to acquire this lock.\nAttempts to acquire this mutex without blocking. Returns …\nAttempts to acquire this lock.\nAttempts to acquire an exclusive lock without blocking.\nAttempts to acquire an exclusive lock until a timeout is …\nAttempts to acquire an exclusive lock until a timeout is …\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire a shared lock without blocking.\nAttempts to acquire a shared lock until a timeout is …\nAttempts to acquire a shared lock without deadlocking in …\nAttempts to acquire a shared lock until a timeout is …\nAttempts to acquire a shared lock until a timeout is …\nAttempts to acquire a shared lock until a timeout is …\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire this lock until a timeout is reached.\nAttempts to acquire an upgradable lock without blocking.\nAttempts to acquire an upgradable lock until a timeout is …\nAttempts to acquire an upgradable lock until a timeout is …\nAttempts to make a new <code>MappedMutexGuard</code> for a component of …\nAttempts to make a new <code>MappedMutexGuard</code> for a component of …\nAttempts to make  a new <code>MappedReentrantMutexGuard</code> for a …\nAttempts to make  a new <code>MappedReentrantMutexGuard</code> for a …\nAttempts to make  a new <code>MappedRwLockReadGuard</code> for a …\nAttempts to make  a new <code>MappedRwLockWriteGuard</code> for a …\nAttempts to make  a new <code>MappedRwLockReadGuard</code> for a …\nAttempts to make  a new <code>MappedRwLockWriteGuard</code> for a …\nAttempts to acquire this <code>RwLock</code> with shared read access.\nAttempts to acquire this <code>RwLock</code> with shared read access …\nAttempts to acquire this <code>RwLock</code> with shared read access.\nAttempts to acquire this <code>RwLock</code> with shared read access …\nAttempts to acquire this <code>RwLock</code> with shared read access …\nAttempts to acquire this <code>RwLock</code> with shared read access …\nAttempts to acquire this <code>RwLock</code> with upgradable read …\nAttempts to acquire this <code>RwLock</code> with upgradable read …\nAttempts to acquire this <code>RwLock</code> with upgradable read …\nAttempts to upgrade an upgradable lock to an exclusive …\nTries to atomically upgrade an upgradable read lock into …\nAttempts to upgrade an upgradable lock to an exclusive …\nTries to atomically upgrade an upgradable read lock into …\nAttempts to upgrade an upgradable lock to an exclusive …\nTries to atomically upgrade an upgradable read lock into …\nFirst, tries to atomically upgrade an upgradable read lock …\nTries to atomically upgrade an upgradable read lock into …\nTries to atomically upgrade an upgradable read lock into …\nAttempts to lock this <code>RwLock</code> with exclusive write access.\nAttempts to acquire this <code>RwLock</code> with exclusive write …\nAttempts to acquire this <code>RwLock</code> with exclusive write …\nUnlocks this mutex.\nUnlocks this mutex. The inner mutex may not be unlocked if …\nReleases an exclusive lock.\nReleases an exclusive lock using a fair unlock protocol.\nUnlocks this mutex using a fair unlock protocol.\nUnlocks this mutex using a fair unlock protocol. The inner …\nUnlocks the mutex using a fair unlock protocol.\nUnlocks the mutex using a fair unlock protocol.\nUnlocks the mutex using a fair unlock protocol.\nUnlocks the mutex using a fair unlock protocol.\nUnlocks the <code>RwLock</code> using a fair unlock protocol.\nUnlocks the <code>RwLock</code> using a fair unlock protocol.\nUnlocks the <code>RwLock</code> using a fair unlock protocol.\nUnlocks the <code>RwLock</code> using a fair unlock protocol.\nUnlocks the <code>RwLock</code> using a fair unlock protocol.\nReleases a shared lock.\nReleases a shared lock using a fair unlock protocol.\nReleases an upgradable lock.\nReleases an upgradable lock using a fair unlock protocol.\nTemporarily unlocks the mutex to execute the given …\nTemporarily unlocks the mutex to execute the given …\nTemporarily unlocks the <code>RwLock</code> to execute the given …\nTemporarily unlocks the <code>RwLock</code> to execute the given …\nTemporarily unlocks the <code>RwLock</code> to execute the given …\nTemporarily unlocks the mutex to execute the given …\nTemporarily unlocks the mutex to execute the given …\nTemporarily unlocks the <code>RwLock</code> to execute the given …\nTemporarily unlocks the <code>RwLock</code> to execute the given …\nTemporarily unlocks the <code>RwLock</code> to execute the given …\nLocks this <code>RwLock</code> with upgradable read access, blocking …\nUpgrades an upgradable lock to an exclusive lock.\nAtomically upgrades an upgradable read lock lock into an …\nFirst, atomically upgrades an upgradable read lock lock …\nLocks this <code>RwLock</code> with exclusive write access, blocking …")