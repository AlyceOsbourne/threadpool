# Threadpool Crate

Welcome to `threadpool`, a crate crafted to simplify parallelism with elegance and clarity. Using a concise set of macros, you can define, run, and manage thread pools without boilerplate. Whether you're spinning up background tasks or orchestrating concurrent computations, this crate aims to make it *just work* (Insert Todd Howard Meme here).

## Examples

Below are several examples that walk through the main features of the crate. These range from fire-and-forget anonymous blocks to manually controlled thread pools, with a syntax that's as flexible as it is expressive.

### Example 1: Auto-sized pool with auto-run

Here, we declare an auto-sized pool. The tasks kick off as soon as they're defined, and the results are unpacked with a whisper of syntactic magic.

```rust
use threadpool::*;

fn main() {
    thread_pool! {
        task1 => {
            println!("[Task 1] Starting");
            sleep!(1); // Assuming a sleep macro or similar exists
            println!("[Task 1] Finished");
            "Hello from task1!"
        },
        task2 => {
            println!("[Task 2] Starting");
            sleep!(2); // Assuming a sleep macro or similar exists
            println!("[Task 2] Finished");
            "Hello from task2!"
        },
    }

    println!("task1 result: {}", task1);
    println!("task2 result: {}", task2);
}
```

### Example 2: Auto-sized without auto-unpacking (requires explicit unpack)

Like the first, but here the `?` prefix indicates that results are *not* automatically unpacked by the `thread_pool!` macro itself. You need a separate `unpack!` call to retrieve the results. It's a minimal, ergonomic way to define tasks that run in parallel when you want more control over when results are accessed.

```rust
use threadpool::*;

fn main() {
    thread_pool! {
        ?
        t1 => {
            println!("[T1] Compute result");
            100
        },
        t2 => {
            println!("[T2] Say hi");
            "Hi from t2"
        },
    }

    unpack!(task1, task2);
    println!("t1 result: {}", t1);
    println!("t2 result: {}", t2);
}
```

### Example 3: Fixed-size pool, named, and run

Sometimes, you want control. This example creates a thread pool with a fixed number of threads and gives it a name. The tasks are queued and launched right away with `@run`.

```rust
use threadpool::*;

fn main() {
    thread_pool!(@run, fixed_pool, 2, {
        a => {
            println!("[A] Running");
        },
        b => {
            println!("[B] Running");
        }
    });
}
```

### Example 4: Manual pool creation and use

Need to hold on to the thread pool and spawn tasks later? This is your pattern. Create it first, spawn tasks when you're ready.

```rust
use threadpool::*;

fn main() {
    thread_pool!(@create, manual_pool, 2);
    spawn_tasks!(manual_pool,
        alpha => {
            println!("[Alpha] Go!");
        },
        beta => {
            println!("[Beta] Go!");
        },
    );
}
```

### Example 5: Anonymous fire-and-forget blocks

Not every task needs a name. Sometimes you just want to do something and move on. This example runs anonymous blocks—useful for fire-and-forget work.

```rust
use threadpool::*;

fn main() {
    thread_pool! {
        {
            println!("[Anon 1] Executing");
        },
        {
            println!("[Anon 2] Executing");
        },
    }

    // You can also mix named and anonymous tasks or use function calls
    fn my_func() {
        println!("hello from my_func!");
    }

    thread_pool!(
        a => { println!("inline block"); },
        b => my_func(),
    );
}
```

---

I mostly created as another ADHD endeavour, so, I hope it is at least useful to someone.
