.PHONY: all generate
all: generate benchmark

generate:
	./generate.sh --internal
	
benchmark: generate
	{ set -e ;\
	cd s2n-tls-sys ;\
	cargo bench --bench drbg  -- --profile-time=5 ;\
	}

# https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md
lldb-server-start:
	bg lldb-server platform --server --listen *:7337

lldb-server-stop:
	pkill lldb-server

