build:
	cd ../../ && JS_SNAPSHOT_BUNDLE=$(realpath ./snapshot.out) cargo build --release --features snapshot_bundle
	gcc test.c ../../target/release/libjs_snap.a -I../../src/include -lm -ldl -lpthread

snapshot:
	cargo run --manifest-path ../../create_snapshot/Cargo.toml ./bundle.js ./snapshot.out

test: snapshot build
	cd ../../ && cargo test -- --nocapture
	./a.out ./snapshot.out