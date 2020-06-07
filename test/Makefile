build:
	cd ../ && cargo build --release
	gcc test.c ../target/release/libjs_snap.a -I../src/include -lm -ldl -lpthread

snapshot:
	cargo run --bin create_snapshot ./bundle.js ./snapshot.out	

test: build snapshot
	cd ../ && cargo test -- --nocapture
	./a.out ./snapshot.out