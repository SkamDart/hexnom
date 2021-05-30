
```bash
[nix-shell:~/skamdart/hexnom]$ python python/main.py 
Traceback (most recent call last):
  File "python/main.py", line 1, in <module>
    import hexnom
ImportError: dynamic module does not define module export function (PyInit_hexnom)
```

:thinkies

[stackoverflow](https://stackoverflow.com/questions/4514745/how-do-i-view-the-list-of-functions-a-linux-shared-library-is-exporting)

```bash
[nix-shell:~/skamdart/hexnom]$ nm -D python/hexnom.so 
                 U PyBytes_AsString
                 U PyBytes_Size
                 U PyCFunction_NewEx
                 U PyDict_Copy
                 U PyDict_DelItem
                 U PyDict_GetItem
                 U PyDict_Next
                 U PyDict_Size
                 U PyErr_Fetch
                 U PyErr_GivenExceptionMatches
                 U PyErr_NewException
                 U PyErr_NormalizeException
                 U PyErr_Print
                 U PyErr_PrintEx
                 U PyErr_Restore
                 U PyEval_SaveThread
                 U PyEval_ThreadsInitialized
                 U PyExc_AttributeError
                 U PyExc_BaseException
                 U PyExc_SystemError
                 U PyExc_TypeError
                 U PyGILState_Ensure
                 U PyGILState_Release
000000000000b3f0 T PyInit_rustlib
                 U PyList_Append
                 U PyList_New
                 U PyLong_FromLong
                 U PyModule_Create2
                 U PyModule_GetName
                 U PyObject_GetAttr
                 U PyObject_Repr
                 U PyObject_SetAttr
                 U PyObject_Str
                 U PyTuple_GetItem
                 U PyTuple_GetSlice
                 U PyTuple_New
                 U PyTuple_SetItem
                 U PyTuple_Size
                 U PyUnicodeDecodeError_Create
                 U PyUnicode_AsEncodedString
                 U PyUnicode_AsUTF8AndSize
                 U PyUnicode_FromStringAndSize
                 U Py_InitializeEx
                 U Py_IsInitialized
                 w _ITM_deregisterTMCloneTable
                 w _ITM_registerTMCloneTable
                 U _Py_Dealloc
                 U _Unwind_Backtrace
                 U _Unwind_DeleteException
                 U _Unwind_GetDataRelBase
                 U _Unwind_GetIP
                 U _Unwind_GetIPInfo
                 U _Unwind_GetLanguageSpecificData
                 U _Unwind_GetRegionStart
                 U _Unwind_GetTextRelBase
                 U _Unwind_RaiseException
                 U _Unwind_Resume
                 U _Unwind_SetGR
                 U _Unwind_SetIP
                 w __cxa_finalize
                 w __cxa_thread_atexit_impl
                 U __errno_location
                 U __fxstat64
                 w __gmon_start__
                 U __tls_get_addr
                 U __xpg_strerror_r
                 U abort
                 U bcmp
                 U calloc
                 U clock_gettime
                 U close
                 U dl_iterate_phdr
                 U dlsym
                 U free
                 U getcwd
                 U getenv
                 U malloc
                 U memchr
                 U memcpy
                 U memmove
                 U memset
                 U mmap
                 U munmap
                 U open64
                 U posix_memalign
                 U pthread_getspecific
                 U pthread_key_create
                 U pthread_key_delete
                 U pthread_mutex_destroy
                 U pthread_mutex_init
                 U pthread_mutex_lock
                 U pthread_mutex_unlock
                 U pthread_mutexattr_destroy
                 U pthread_mutexattr_init
                 U pthread_mutexattr_settype
                 U pthread_rwlock_rdlock
                 U pthread_rwlock_unlock
                 U pthread_setspecific
                 U readlink
                 U realloc
000000000002f720 T rust_eh_personality
                 U sched_yield
                 U strlen
                 U syscall
                 U write
                 U writev
```

Ahhh.... It has a PyInit_rustlib.... Does it expect that?

Nah. See function with attribute `#[pymodinit]` in `src/lib.rs`

