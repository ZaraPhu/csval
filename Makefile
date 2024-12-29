VERSION=0.1
EXEC-NAME=csval
INSTALL-LOCATION=/usr/local/bin

install:
	@echo "Building release version $(VERSION)..."
	@cargo build --release	
	@echo "Moving executable to $(INSTALL-LOCATION)/$(EXEC-NAME)"
	@sudo cp "./target/release/$(EXEC-NAME)" "$(INSTALL-LOCATION)"

clean:
	@echo "Cleaning build dir..."
	@rm -rf target
	@echo "Running cargo clean..."
	@cargo clean

uninstall:
	@echo "Deleting $(INSTALL-LOCATION)/$(EXEC-NAME)"
	@sudo rm "$(INSTALL-LOCATION)/$(EXEC-NAME)"
