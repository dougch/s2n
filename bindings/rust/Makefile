.PHONY: all generate clean
all: generate benchmark

generate: clean
	./generate.sh --internal
	
benchmark: generate
	{ set -e ;\
	cd s2n-tls-sys ;\
	cargo bench --bench drbg  -- --profile-time=5 ;\
	}

clean:
	@rm -rf s2n-tls-sys/src/internal.rs

# https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md
lldb-server-start:
	bg lldb-server platform --server --listen *:7337

lldb-server-stop:
	pkill lldb-server

