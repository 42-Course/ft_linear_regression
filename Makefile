build:
	@cargo build

predictor:
	@cargo run --package predictor

trainer:
	@cargo run --package trainer

gui:
	@cargo run --package gui
	
web:
	trunk serve --config gui/

clean:
	@cargo clean

.PHONY: predictor trainer gui