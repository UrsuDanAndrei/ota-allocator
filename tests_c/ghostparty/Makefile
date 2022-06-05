LIB_LOCATION=$(pwd)/../target/release

all: build

build: ghostparty ghostparty_debug ghostparty_regular

ghostparty: ghostparty.cpp
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/release \
	g++ ghostparty.cpp -o ghostparty \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue  \
		-L/home/dan/workspace/licenta/ota-allocator/target/release \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

ghostparty_debug: ghostparty.cpp
	# YOU ALSO NEED TO export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug
	LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/dan/workspace/licenta/ota-allocator/target/debug \
	g++ ghostparty.cpp -o ghostparty_debug \
		-I/home/dan/workspace/licenta/ota-allocator/c_glue \
		-L/home/dan/workspace/licenta/ota-allocator/target/debug \
		-lota_allocator -lpthread \
		-Wl,--no-as-needed -ldl

ghostparty_regular: ghostparty.cpp
	g++ ghostparty.cpp -o ghostparty_regular -I/home/dan/workspace/licenta/ota-allocator/c_glue

clean:
	rm -f ghostparty ghostparty_debug ghostparty_regular
