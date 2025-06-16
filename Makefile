.PHONY: build clean run

run: # to directly run the excecutable in profan
	mv build/link/prog build/ProfanOS/sys_dir/user/prog
	make -C build/ProfanOS disk run

build:
	make clean

	mkdir build
	
	git clone https://github.com/elydre/profanOS build/ProfanOS

	make -C build/ProfanOS disk
	mkdir build/link
	cp build/ProfanOS/out/zlibs/libc.so build/link/libc.so
	cargo build
	mv target/i386/debug/testrust build/link/prog

clean:
	rm -f -Rf build
	rm -f -Rf target
