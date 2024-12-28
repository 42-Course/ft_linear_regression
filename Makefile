build:
	cargo build

predictor:
	cargo run --package predictor

trainer:
	cargo run --package trainer

gui:
	cargo run --package gui
	
clean: clean_gui clean_trainer clean_predictor
	cargo clean

clean_gui:
	cargo clean --target-dir gui

clean_trainer:
	cargo clean --target-dir trainer

clean_predictor:
	cargo clean --target-dir predictor

.PHONY: predictor trainer gui