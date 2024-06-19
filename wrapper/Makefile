# Define variables
TARGET ?= 42
ASSEMBLY_SOURCE = $(TARGET).s
ASSEMBLY_OBJECT = $(TARGET).o
STATIC_LIB = libcompiled_code.a
RUNNABLE = $(TARGET).run
RUST_WRAPPER = stub.rs

# Default target
all: run clean

# Generate assembly file from Rust source
$(ASSEMBLY_SOURCE): $(RUST_WRAPPER)
	cargo  run $(TARGET).int > $(ASSEMBLY_SOURCE)

# Compile assembly to object file
$(ASSEMBLY_OBJECT): $(ASSEMBLY_SOURCE)
	nasm -f macho64 -o $(ASSEMBLY_OBJECT) $(ASSEMBLY_SOURCE)

# Create a static library from the object file
$(STATIC_LIB): $(ASSEMBLY_OBJECT)
	ar r $(STATIC_LIB) $(ASSEMBLY_OBJECT)

# Compile the Rust stub, link with the static library, and create an executable
$(RUNNABLE): $(STATIC_LIB)
	rustc $(RUST_WRAPPER) -L . --target=x86_64-apple-darwin -o $(RUNNABLE)

# Run the compiled executable
run: $(RUNNABLE)
	@./$(RUNNABLE)

# Clean up build artifacts
clean:
	@rm -f $(ASSEMBLY_SOURCE) $(ASSEMBLY_OBJECT) $(STATIC_LIB) $(RUNNABLE)

.PHONY: all run clean
