# expr-std-channels

Experiment with std::channels. I answered two questions:

 * Channels can be used to send data between items on the same thread.
 * That scoping of threads causes them to close.
 * A `tx.send()` can be used **without** an "active" `rx.recv()`
 * `tx.send()` is non-blocking `rx.recv()` blocks if no messages, from
   the documentation there is a `try_recv()`
 * `thread_join_handle.join()` is used to "wait" for a thread to stop
   and the `thread_join_handle` is returned by `std::thread::spawn`.

## Building and running

Output is via `log::info!()` and `log::debug!()` statements with the
default being `log::debug!()`:

```
wink@3900x 22-05-06T23:02:56.741Z:~/prgs/rust/myrepos/expr-std-channels (main)
$ cargo run --release
   Compiling expr-std-channels v0.1.0 (/home/wink/prgs/rust/myrepos/expr-std-channels)
    Finished release [optimized] target(s) in 0.42s
     Running `target/release/expr-std-channels`
[2022-05-06T23:22:17.306414698Z INFO  expr_std_channels   76  1] main:+
[2022-05-06T23:22:17.306544588Z INFO  expr_std_channels   80  1] main:- 131260ns
wink@3900x 22-05-06T23:22:17.307Z:~/prgs/rust/myrepos/expr-std-channels (main)
```

You can also see more detail with `RUST_LOG=debug`:
```
wink@3900x 22-05-06T23:22:17.307Z:~/prgs/rust/myrepos/expr-std-channels (main)
$ RUST_LOG=debug cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/expr-std-channels`
[2022-05-06T23:22:49.896202991Z INFO  expr_std_channels   76  1] main:+
[2022-05-06T23:22:49.896209041Z DEBUG expr_std_channels    9  1] do_work:+
[2022-05-06T23:22:49.896210211Z DEBUG expr_std_channels   14  1] do_work: inner-scope-
[2022-05-06T23:22:49.896273001Z DEBUG expr_std_channels   48  1] do_work: send(132::MAX)-;inner-scope-
[2022-05-06T23:22:49.896319651Z DEBUG expr_std_channels   32  3] Reciever thread:+
[2022-05-06T23:22:49.896333111Z DEBUG expr_std_channels   35  3] v=1
[2022-05-06T23:22:49.896335451Z DEBUG expr_std_channels   35  3] v=2147483647
[2022-05-06T23:22:49.896338691Z DEBUG expr_std_channels   24  2] Sender thread:+
[2022-05-06T23:22:49.896355621Z DEBUG expr_std_channels   28  2] Sender thread:-
[2022-05-06T23:22:49.896365821Z DEBUG expr_std_channels   35  3] v=100
[2022-05-06T23:22:49.896369061Z DEBUG expr_std_channels   35  3] v=101
[2022-05-06T23:22:49.896370461Z DEBUG expr_std_channels   35  3] v=102
[2022-05-06T23:22:49.896371971Z DEBUG expr_std_channels   37  3] err=RecvError
[2022-05-06T23:22:49.896375331Z DEBUG expr_std_channels   42  3] Reciever thread:-
[2022-05-06T23:22:49.896390181Z DEBUG expr_std_channels   56  1] do_work:-
[2022-05-06T23:22:49.896393011Z INFO  expr_std_channels   80  1] main:- 190940ns
wink@3900x 22-05-06T23:22:49.897Z:~/prgs/rust/myrepos/expr-std-channels (main)
```

And using `RUST_LOG=` will supress the output:
```
wink@3900x 22-05-06T23:22:49.897Z:~/prgs/rust/myrepos/expr-std-channels (main)
$ RUST_LOG= cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/expr-std-channels`
wink@3900x 22-05-06T23:23:21.765Z:~/prgs/rust/myrepos/expr-std-channels (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
