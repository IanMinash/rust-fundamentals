# Memory Management

## 1. Stack

The stack is a special region in process memory used for storing variables
created by each function.

Only the function that created a stack has access to it thus enabling
local scope.

When a function is called, a new stack frame is created on top of the
current one. When the ffunction execution is done, the stack frame is
realeased from memory.

The size of each stack frame is determined at compile time because the
types of each variable are known.

## 2. Heap

The heap is a region is process memory, which unlike the stack, is not
automatically managed.
It has no size restrictions and is only limited by the physical resources of
the machine the process is running on.

The heap can also be accessed by any function in any part of the program. Heap
allocations are expensive and should therefore be avoided. This is because the
heap is usually fragmented in nature because the process has to find a space
in memory big enough to hold the data being allocated.

Since the heap is manually managed, there exists a possibility of having a
memory leak in the case that a stack frame containing a pointer is released yet
the data stored in that memory address hasn't been deallocated. This will
remain in memory until the program exits. It is therefore best practice to
deallocate memory allocated in the heap.

## 3. Smart Pointers

Smart pointers are wrappers around normal pointers which have convenient
utilities such as automatically deallocating the memory address they are
referencing to when they go out of scope.

The `Box` provided by Rust is a smart pointer. It automatically deallocates
the memory being used when it is taken out of scope.
