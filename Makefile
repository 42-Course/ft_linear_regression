build:
	@cargo build

predictor:
	@cargo run --package predictor

trainer:
	@cargo run --package trainer

gui:
	@cargo run --package gui
	
clean: clean_gui clean_trainer clean_predictor
	@cargo clean

clean_gui:
	@cargo clean --target gui

clean_trainer:
	@cargo clean --target trainer

clean_predictor:
	@cargo clean --target predictor

.PHONY: predictor trainer gui