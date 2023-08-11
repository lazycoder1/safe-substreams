ENDPOINT ?= mainnet.eth.streamingfast.io:443
	
.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_safe_multi_sig_transactions -s 17600083 -t +10000

.PHONY: stream_gui
stream_gui: build
	substreams gui -e $(ENDPOINT) \
		substreams.yaml \
		graph_out \
		--start-block 17890084 \
		--stop-block +2000

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml
